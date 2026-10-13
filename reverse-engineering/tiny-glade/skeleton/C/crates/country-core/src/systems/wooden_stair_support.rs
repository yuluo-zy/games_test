// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wooden_stair_support.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f7fa7ee3d56f4cdf5bf384a0242af1fb5845977f1d8c1c156cfdb46a9f5f705a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wooden_stair_support.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 557, 1682
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::resample_wooden_support_station_points
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":19404,"rva":11474112,"va":5380183232}
// 编译实例 RVAs: 0xaf14c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4504617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8d743bbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380183300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4504782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e83243bbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380183481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11461100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8141e51ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380183287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081c20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10943039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8c10559ff","size":5,"soft_fail":false,"target_va":5369240608,"va":5380183642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10125064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8087f9a00","size":5,"soft_fail":false,"target_va":5390308448,"va":5380183379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196e460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15191747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8c3cee700","size":5,"soft_fail":false,"target_va":5395375200,"va":5380183448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36873720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8f8a53202","size":5,"soft_fail":false,"target_va":5417057056,"va":5380183331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36437184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380183232,"raw_hex":"e8c0fc2b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380183605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":19584,"parent_record_offset":19404,"ancestor_record_offset":19404,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":557,"record_offset":20744,"parent_record_offset":19404,"ancestor_record_offset":19404,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":557,"record_offset":20028,"parent_record_offset":19404,"ancestor_record_offset":19404,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":557,"record_offset":20708,"parent_record_offset":19404,"ancestor_record_offset":19404,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19828,"parent_record_offset":19804,"ancestor_record_offset":19404,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":19404,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":19404,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":19404,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19876,"parent_record_offset":19852,"ancestor_record_offset":19404,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19404,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":19404,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20276,"parent_record_offset":20184,"ancestor_record_offset":19404,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":19712,"parent_record_offset":19680,"ancestor_record_offset":19404,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20212,"parent_record_offset":20184,"ancestor_record_offset":19404,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":19404,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":19772,"parent_record_offset":19740,"ancestor_record_offset":19404,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":19740,"parent_record_offset":19712,"ancestor_record_offset":19404,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":19404,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":19612,"parent_record_offset":19584,"ancestor_record_offset":19404,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":19640,"parent_record_offset":19612,"ancestor_record_offset":19404,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":19988,"parent_record_offset":19640,"ancestor_record_offset":19404,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":19680,"parent_record_offset":19640,"ancestor_record_offset":19404,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":20244,"parent_record_offset":20212,"ancestor_record_offset":19404,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":557,"record_offset":19932,"parent_record_offset":19640,"ancestor_record_offset":19404,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":19956,"parent_record_offset":19932,"ancestor_record_offset":19404,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":19404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":20060,"parent_record_offset":20028,"ancestor_record_offset":19404,"inlinee":34833,"source_line":53,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":20776,"parent_record_offset":20744,"ancestor_record_offset":19404,"inlinee":34834,"source_line":55,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8811
// 首条位置: {"module":557,"inlinee":34833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":20060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":20060,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":20060,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":20060,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20276,"parent_record_offset":20184,"ancestor_record_offset":20060,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20212,"parent_record_offset":20184,"ancestor_record_offset":20060,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":20060,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":20060,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":20244,"parent_record_offset":20212,"ancestor_record_offset":20060,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":20112,"parent_record_offset":20088,"ancestor_record_offset":20060,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8812
// 首条位置: {"module":557,"inlinee":34834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":20800,"parent_record_offset":20776,"ancestor_record_offset":20776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20984,"parent_record_offset":20960,"ancestor_record_offset":20776,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":21008,"parent_record_offset":20984,"ancestor_record_offset":20776,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":21036,"parent_record_offset":21008,"ancestor_record_offset":20776,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":20960,"parent_record_offset":20872,"ancestor_record_offset":20776,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20900,"parent_record_offset":20872,"ancestor_record_offset":20776,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20872,"parent_record_offset":20848,"ancestor_record_offset":20776,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":20848,"parent_record_offset":20824,"ancestor_record_offset":20776,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":20928,"parent_record_offset":20900,"ancestor_record_offset":20776,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":20824,"parent_record_offset":20800,"ancestor_record_offset":20776,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::inset_wooden_stair_terminal_station_points
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":21104,"rva":11474544,"va":5380183664}
// 编译实例 RVAs: 0xaf1670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21368,"parent_record_offset":21104,"ancestor_record_offset":21104,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21612,"parent_record_offset":21104,"ancestor_record_offset":21104,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21668,"parent_record_offset":21104,"ancestor_record_offset":21104,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21288,"parent_record_offset":21104,"ancestor_record_offset":21104,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":21104,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":21104,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":21696,"parent_record_offset":21668,"ancestor_record_offset":21104,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":21316,"parent_record_offset":21288,"ancestor_record_offset":21104,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21500,"parent_record_offset":21464,"ancestor_record_offset":21104,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":21104,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21436,"parent_record_offset":21400,"ancestor_record_offset":21104,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":21400,"parent_record_offset":21368,"ancestor_record_offset":21104,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":21536,"parent_record_offset":21500,"ancestor_record_offset":21104,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::plan_braces_between_posts
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":14856,"rva":11470768,"va":5380179888}
// 编译实例 RVAs: 0xaf07b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4501877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e88b4ebbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380180560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4504201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e87745bbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380182900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11458358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e8ca2851ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380180545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972e40
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1565790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e8a21be8ff","size":5,"soft_fail":false,"target_va":5378616896,"va":5380182681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3144737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e821fc2f00","size":5,"soft_fail":false,"target_va":5383325056,"va":5380180314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36877017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e8d9b23202","size":5,"soft_fail":false,"target_va":5417057536,"va":5380180514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36438274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e802012c02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380182490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec::push<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e85b320000","size":5,"soft_fail":false,"target_va":5380193824,"va":5380180928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36439680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e880062c02","size":5,"soft_fail":false,"target_va":5416622240,"va":5380182555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wooden_stair_support::curved_brace_placement
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e8bcf5ffff","size":5,"soft_fail":false,"target_va":5380178352,"va":5380180975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::brace::quarter_ellipse_brace
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10024507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380179888,"raw_hex":"e83bf69800","size":5,"soft_fail":false,"target_va":5390205952,"va":5380181440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::systems::wooden_stair_support::plan_curved_brace
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":18408,"parent_record_offset":14856,"ancestor_record_offset":14856,"inlinee":34829,"source_line":173,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wooden_stair_support::plan_straight_brace
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":15244,"parent_record_offset":14856,"ancestor_record_offset":14856,"inlinee":34826,"source_line":102,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15020,"parent_record_offset":14856,"ancestor_record_offset":14856,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":14856,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":14856,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":14856,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19156,"parent_record_offset":19132,"ancestor_record_offset":14856,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":14856,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19180,"parent_record_offset":19156,"ancestor_record_offset":14856,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":14856,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":14856,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16592,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":14856,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16420,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":14856,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":16616,"parent_record_offset":16592,"ancestor_record_offset":14856,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":16640,"parent_record_offset":16616,"ancestor_record_offset":14856,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":14856,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":14856,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":16336,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":16364,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":557,"record_offset":18572,"parent_record_offset":18524,"ancestor_record_offset":14856,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":557,"record_offset":18600,"parent_record_offset":18572,"ancestor_record_offset":14856,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15708,"parent_record_offset":15404,"ancestor_record_offset":14856,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15680,"parent_record_offset":15404,"ancestor_record_offset":14856,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":14856,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":14856,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":16240,"parent_record_offset":16088,"ancestor_record_offset":14856,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":14856,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":14856,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":15084,"parent_record_offset":15044,"ancestor_record_offset":14856,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":14856,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17716,"parent_record_offset":17648,"ancestor_record_offset":14856,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":19060,"parent_record_offset":18408,"ancestor_record_offset":14856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::horizontal_stringer_attachment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":17648,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":34831,"source_line":274,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::planned_curved_brace
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":18812,"parent_record_offset":18408,"ancestor_record_offset":14856,"inlinee":34830,"source_line":217,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::straight_brace_post_attachment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":15404,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":34827,"source_line":292,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":557,"record_offset":16152,"parent_record_offset":16120,"ancestor_record_offset":14856,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15580,"parent_record_offset":15552,"ancestor_record_offset":14856,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15552,"parent_record_offset":15404,"ancestor_record_offset":14856,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16704,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15112,"parent_record_offset":15044,"ancestor_record_offset":14856,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16120,"parent_record_offset":16088,"ancestor_record_offset":14856,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16088,"parent_record_offset":16060,"ancestor_record_offset":14856,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15828,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17828,"parent_record_offset":17648,"ancestor_record_offset":14856,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16060,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17340,"parent_record_offset":17312,"ancestor_record_offset":14856,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17312,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":14856,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17936,"parent_record_offset":17828,"ancestor_record_offset":14856,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":14856,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17168,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15972,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::acos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":16392,"parent_record_offset":15244,"ancestor_record_offset":14856,"inlinee":34828,"source_line":808,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":15148,"parent_record_offset":15112,"ancestor_record_offset":14856,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::plan_straight_brace
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x880A
// 首条位置: {"module":557,"inlinee":34826}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16592,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16420,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":16336,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":16364,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":16676,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wooden_stair_support::horizontal_stringer_attachment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":17648,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":34831,"source_line":274,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wooden_stair_support::straight_brace_post_attachment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":15404,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":34827,"source_line":292,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16704,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16304,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15828,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16060,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17312,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15740,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17168,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15972,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::acos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":16392,"parent_record_offset":15244,"ancestor_record_offset":15244,"inlinee":34828,"source_line":808,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":15244,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":15244,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":15244,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":15244,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":16444,"parent_record_offset":16420,"ancestor_record_offset":15244,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":557,"record_offset":16616,"parent_record_offset":16592,"ancestor_record_offset":15244,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":557,"record_offset":16640,"parent_record_offset":16616,"ancestor_record_offset":15244,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":15244,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":15244,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":17688,"parent_record_offset":17648,"ancestor_record_offset":15244,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15708,"parent_record_offset":15404,"ancestor_record_offset":15244,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15680,"parent_record_offset":15404,"ancestor_record_offset":15244,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":15244,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":15244,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":16240,"parent_record_offset":16088,"ancestor_record_offset":15244,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":15244,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":15244,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":15492,"parent_record_offset":15460,"ancestor_record_offset":15244,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":15244,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17716,"parent_record_offset":17648,"ancestor_record_offset":15244,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":557,"record_offset":16152,"parent_record_offset":16120,"ancestor_record_offset":15244,"inlinee":12080,"source_line":192,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15580,"parent_record_offset":15552,"ancestor_record_offset":15244,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15552,"parent_record_offset":15404,"ancestor_record_offset":15244,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":15244,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16120,"parent_record_offset":16088,"ancestor_record_offset":15244,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":16088,"parent_record_offset":16060,"ancestor_record_offset":15244,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17828,"parent_record_offset":17648,"ancestor_record_offset":15244,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17340,"parent_record_offset":17312,"ancestor_record_offset":15244,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":15244,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17936,"parent_record_offset":17828,"ancestor_record_offset":15244,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15460,"parent_record_offset":15404,"ancestor_record_offset":15244,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":15616,"parent_record_offset":15580,"ancestor_record_offset":15244,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::plan_curved_brace
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x880D
// 首条位置: {"module":557,"inlinee":34829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":18640,"parent_record_offset":18408,"ancestor_record_offset":18408,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":18524,"parent_record_offset":18408,"ancestor_record_offset":18408,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":19060,"parent_record_offset":18408,"ancestor_record_offset":18408,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wooden_stair_support::planned_curved_brace
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":18812,"parent_record_offset":18408,"ancestor_record_offset":18408,"inlinee":34830,"source_line":217,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":18712,"parent_record_offset":18688,"ancestor_record_offset":18408,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":18408,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19132,"parent_record_offset":19108,"ancestor_record_offset":18408,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19156,"parent_record_offset":19132,"ancestor_record_offset":18408,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":18408,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19180,"parent_record_offset":19156,"ancestor_record_offset":18408,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":18688,"parent_record_offset":18664,"ancestor_record_offset":18408,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":19108,"parent_record_offset":19084,"ancestor_record_offset":18408,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":18664,"parent_record_offset":18640,"ancestor_record_offset":18408,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":557,"record_offset":19084,"parent_record_offset":19060,"ancestor_record_offset":18408,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":557,"record_offset":18572,"parent_record_offset":18524,"ancestor_record_offset":18408,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":557,"record_offset":18600,"parent_record_offset":18572,"ancestor_record_offset":18408,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::planned_curved_brace
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x880E
// 首条位置: {"module":557,"inlinee":34830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::curved_brace_placement
// 记录形式: compiled_procedure; instances=1
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":13404,"rva":11469232,"va":5380178352}
// 编译实例 RVAs: 0xaf01b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4501168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380178352,"raw_hex":"e85051bbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380179867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden_stair_support.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3146469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380178352,"raw_hex":"e8e5023000","size":5,"soft_fail":false,"target_va":5383325056,"va":5380178582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::brace::quarter_ellipse_brace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10027304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380178352,"raw_hex":"e828019900","size":5,"soft_fail":false,"target_va":5390205952,"va":5380178643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":557,"record_offset":13780,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":557,"record_offset":13744,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":14544,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":13624,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":13572,"parent_record_offset":13404,"ancestor_record_offset":13404,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":14748,"parent_record_offset":14724,"ancestor_record_offset":13404,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":13404,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":13404,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":557,"record_offset":14724,"parent_record_offset":14628,"ancestor_record_offset":13404,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":14660,"parent_record_offset":14628,"ancestor_record_offset":13404,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":13404,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":557,"record_offset":14600,"parent_record_offset":14572,"ancestor_record_offset":13404,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":14136,"parent_record_offset":13924,"ancestor_record_offset":13404,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":14360,"parent_record_offset":13924,"ancestor_record_offset":13404,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":13404,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":557,"record_offset":13896,"parent_record_offset":13804,"ancestor_record_offset":13404,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":557,"record_offset":14692,"parent_record_offset":14660,"ancestor_record_offset":13404,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":14320,"parent_record_offset":14176,"ancestor_record_offset":13404,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":14248,"parent_record_offset":14176,"ancestor_record_offset":13404,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":13404,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":557,"record_offset":14400,"parent_record_offset":13896,"ancestor_record_offset":13404,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":557,"record_offset":14572,"parent_record_offset":14544,"ancestor_record_offset":13404,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":557,"record_offset":13868,"parent_record_offset":13804,"ancestor_record_offset":13404,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":557,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":13404,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wooden_stair_support::curved_brace_placement::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wooden_stair_support.rs
//     evidence={"module":557,"record_offset":13924,"parent_record_offset":13896,"ancestor_record_offset":13404,"inlinee":34824,"source_line":249,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":13404,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":13984,"parent_record_offset":13924,"ancestor_record_offset":13404,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":557,"record_offset":14176,"parent_record_offset":13924,"ancestor_record_offset":13404,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":13680,"parent_record_offset":13648,"ancestor_record_offset":13404,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::curved_brace_placement::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8808
// 首条位置: {"module":557,"inlinee":34824}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":14136,"parent_record_offset":13924,"ancestor_record_offset":13924,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":14360,"parent_record_offset":13924,"ancestor_record_offset":13924,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":14100,"parent_record_offset":13924,"ancestor_record_offset":13924,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":13984,"parent_record_offset":13924,"ancestor_record_offset":13924,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":557,"record_offset":14176,"parent_record_offset":13924,"ancestor_record_offset":13924,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":14320,"parent_record_offset":14176,"ancestor_record_offset":13924,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":14248,"parent_record_offset":14176,"ancestor_record_offset":13924,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":14028,"parent_record_offset":13984,"ancestor_record_offset":13924,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::horizontal_stringer_attachment
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x880F
// 首条位置: {"module":557,"inlinee":34831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":17688,"parent_record_offset":17648,"ancestor_record_offset":17648,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17716,"parent_record_offset":17648,"ancestor_record_offset":17648,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17828,"parent_record_offset":17648,"ancestor_record_offset":17648,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":17648,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":557,"record_offset":17788,"parent_record_offset":17764,"ancestor_record_offset":17648,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":17648,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":17876,"parent_record_offset":17852,"ancestor_record_offset":17648,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":557,"record_offset":17740,"parent_record_offset":17716,"ancestor_record_offset":17648,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":17648,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":17936,"parent_record_offset":17828,"ancestor_record_offset":17648,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wooden_stair_support::straight_brace_post_attachment
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x880B
// 首条位置: {"module":557,"inlinee":34827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15708,"parent_record_offset":15404,"ancestor_record_offset":15404,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15680,"parent_record_offset":15404,"ancestor_record_offset":15404,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15552,"parent_record_offset":15404,"ancestor_record_offset":15404,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":557,"record_offset":15460,"parent_record_offset":15404,"ancestor_record_offset":15404,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":557,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":15404,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":557,"record_offset":15492,"parent_record_offset":15460,"ancestor_record_offset":15404,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":557,"record_offset":15580,"parent_record_offset":15552,"ancestor_record_offset":15404,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":557,"record_offset":15616,"parent_record_offset":15580,"ancestor_record_offset":15404,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00557.jsonl.gz"}
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
// 完整结构及原始类型记录: wooden_stair_support.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>", "constructor": "enum2$", "arguments": ["country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::push<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::push", "arguments": ["enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<enum2$<country_core::systems::wooden_stair_support::PlannedWoodenSupportPiece>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wooden_stair_support.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wooden_stair_support.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::curved_brace_placement
// 信息 ID: a49188840508ed83c74529c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wooden_stair_support::plan_braces_between_posts @ 0xaf0bef -> 0xaf01b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wooden_stair_support::plan_braces_between_posts @ 0xaf0c82 -> 0xaf01b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::curved_brace_placement::closure$0
// 信息 ID: 1dda8376f5aa74a87e109b8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wooden_stair_support::curved_brace_placement", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::curved_brace_placement; module=557, inline_record=13924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::horizontal_stringer_attachment
// 信息 ID: cbc52e6292c1b1aed8fb31a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=17648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::inset_wooden_stair_terminal_station_points
// 信息 ID: 9d9320cfb7b40dc6ff9bc547
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports @ 0x823989 -> 0xaf1670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_wooden_railings @ 0x8b33a7 -> 0xaf1670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::plan_braces_between_posts
// 信息 ID: 778633690ee7cc8a0b6ea763
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::plan_braces_to_attachment_tops @ 0x822a56 -> 0xaf07b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::plan_curved_brace
// 信息 ID: f483deb435d93e31bef70fa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=18408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::plan_straight_brace
// 信息 ID: 59f0c267b68826e02e78928c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=15244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::planned_curved_brace
// 信息 ID: e7cdff771077f91ee1a83fbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=18812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=19020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::resample_wooden_support_station_points
// 信息 ID: 730bb679efbe7b34e8973526
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::wooden_supports::construct_playermade_wooden_stair_supports @ 0x82385b -> 0xaf14c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::playermade_stairs::utils::stairs_wooden_railings_assemble::assemble_deferred_wooden_railings @ 0x8b8134 -> 0xaf14c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$0
// 信息 ID: 69bb454f8f2348250bea457d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wooden_stair_support::resample_wooden_support_station_points", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::resample_wooden_support_station_points; module=557, inline_record=20060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::resample_wooden_support_station_points::closure$1
// 信息 ID: 17b35ed0ced53525a2afa935
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wooden_stair_support::resample_wooden_support_station_points", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::resample_wooden_support_station_points; module=557, inline_record=20776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wooden_stair_support::straight_brace_post_attachment
// 信息 ID: 626cd161a3812d7e4af06063
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=15404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wooden_stair_support::plan_braces_between_posts; module=557, inline_record=16856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
