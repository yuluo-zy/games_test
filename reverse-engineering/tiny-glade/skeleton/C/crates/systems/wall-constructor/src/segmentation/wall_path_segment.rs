// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1bf3f6aaf0ac9da2e1b45445737aba98b216afec712c4e530e397fab7c711fbd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 394, 410, 1460, 1471
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65ED
// 首条位置: {"module":394,"inlinee":26093}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":394,"record_offset":16232,"parent_record_offset":16180,"ancestor_record_offset":16180,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1460,"record_offset":560,"rva":21005824,"va":5389714944}
// 编译实例 RVAs: 0x1408600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14036737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8ffd029ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389715436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140082d10
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20470509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e813a5c7fe","size":5,"soft_fail":false,"target_va":5369244944,"va":5389715448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de61f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6435211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e875ce9dff","size":5,"soft_fail":false,"target_va":5383283184,"va":5389718390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6434750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e842d09dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389718361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f5650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-78652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8c4ccfeff","size":5,"soft_fail":false,"target_va":5389637200,"va":5389715847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f8420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-69027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e85df2feff","size":5,"soft_fail":false,"target_va":5389648928,"va":5389717950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fc440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8d532ffff","size":5,"soft_fail":false,"target_va":5389665344,"va":5389717862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054a0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e865c9ffff","size":5,"soft_fail":false,"target_va":5389702304,"va":5389716278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140b850
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8781d0000","size":5,"soft_fail":false,"target_va":5389727824,"va":5389720275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142e580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t152421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e865530200","size":5,"soft_fail":false,"target_va":5389870464,"va":5389718038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26650635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e80ba89601","size":5,"soft_fail":false,"target_va":5416365968,"va":5389715328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=21
//     evidence={"assembly":"\tcallq\t26902527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8ff7f9a01","size":5,"soft_fail":false,"target_va":5416621762,"va":5389719230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26903013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8e5819a01","size":5,"soft_fail":false,"target_va":5416622560,"va":5389719542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26903271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e8e7829a01","size":5,"soft_fail":false,"target_va":5416622624,"va":5389719348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26903100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e83c829a01","size":5,"soft_fail":false,"target_va":5416622240,"va":5389719135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e885c6ffff","size":5,"soft_fail":false,"target_va":5389701616,"va":5389716326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e88abdffff","size":5,"soft_fail":false,"target_va":5389701760,"va":5389718769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389714944,"raw_hex":"e82bbfffff","size":5,"soft_fail":false,"target_va":5389702160,"va":5389718752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":812,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":1096,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":6040,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":6232,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1460,"record_offset":3960,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1460,"record_offset":980,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1460,"record_offset":3988,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1460,"record_offset":2468,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1460,"record_offset":2440,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2356,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":5928,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":11853,"source_line":3491,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":3876,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$119::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1460,"record_offset":5704,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":11007,"source_line":27,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1460,"record_offset":1332,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1460,"record_offset":896,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1460,"record_offset":3792,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1460,"record_offset":1948,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":1460,"record_offset":6328,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":79454,"source_line":126,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1460,"record_offset":840,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":26122,"source_line":175,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] utils::slice_iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1460,"record_offset":2300,"parent_record_offset":560,"ancestor_record_offset":560,"inlinee":79450,"source_line":674,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1460,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":560,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1460,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":560,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1460,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":560,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1460,"record_offset":1496,"parent_record_offset":1404,"ancestor_record_offset":560,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1180,"parent_record_offset":1156,"ancestor_record_offset":560,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":560,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":560,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":560,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":560,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1460,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":560,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":1156,"parent_record_offset":1124,"ancestor_record_offset":560,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":560,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1460,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":560,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1460,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":560,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1460,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":560,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1460,"record_offset":2576,"parent_record_offset":2524,"ancestor_record_offset":560,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1460,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":560,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1460,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":560,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::Copied::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1460,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":560,"inlinee":11855,"source_line":24,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":1460,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":560,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1460,"record_offset":4324,"parent_record_offset":4172,"ancestor_record_offset":560,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1460,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":560,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1460,"record_offset":4028,"parent_record_offset":3988,"ancestor_record_offset":560,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1460,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":560,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1460,"record_offset":4068,"parent_record_offset":4028,"ancestor_record_offset":560,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1460,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":560,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2800,"parent_record_offset":2760,"ancestor_record_offset":560,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2828,"parent_record_offset":2800,"ancestor_record_offset":560,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2760,"parent_record_offset":2524,"ancestor_record_offset":560,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2856,"parent_record_offset":2828,"ancestor_record_offset":560,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1460,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":560,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1460,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":560,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1460,"record_offset":2704,"parent_record_offset":2524,"ancestor_record_offset":560,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1460,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":560,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1460,"record_offset":6440,"parent_record_offset":6396,"ancestor_record_offset":560,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1460,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":560,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1460,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":560,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1460,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":560,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1460,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":560,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1460,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":560,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1460,"record_offset":4964,"parent_record_offset":4172,"ancestor_record_offset":560,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1460,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":560,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1460,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":560,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1460,"record_offset":1060,"parent_record_offset":1004,"ancestor_record_offset":560,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1460,"record_offset":1032,"parent_record_offset":1004,"ancestor_record_offset":560,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1460,"record_offset":1268,"parent_record_offset":1124,"ancestor_record_offset":560,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1460,"record_offset":6068,"parent_record_offset":6040,"ancestor_record_offset":560,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1460,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":560,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1460,"record_offset":6096,"parent_record_offset":6068,"ancestor_record_offset":560,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1460,"record_offset":6288,"parent_record_offset":6260,"ancestor_record_offset":560,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1460,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":560,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1460,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":560,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1460,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":560,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1460,"record_offset":4228,"parent_record_offset":4172,"ancestor_record_offset":560,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1460,"record_offset":2068,"parent_record_offset":1948,"ancestor_record_offset":560,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1460,"record_offset":6580,"parent_record_offset":6328,"ancestor_record_offset":560,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1460,"record_offset":6608,"parent_record_offset":6328,"ancestor_record_offset":560,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1460,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":560,"inlinee":79452,"source_line":661,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1460,"record_offset":2604,"parent_record_offset":2524,"ancestor_record_offset":560,"inlinee":79333,"source_line":212,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$6::next::closure$0
//     owner=private_source_ownership_candidate; count=12
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1460,"record_offset":2664,"parent_record_offset":2632,"ancestor_record_offset":560,"inlinee":79335,"source_line":213,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1460,"record_offset":6396,"parent_record_offset":6328,"ancestor_record_offset":560,"inlinee":26187,"source_line":226,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1365E
// 首条位置: {"module":1460,"inlinee":79454}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::comparative_space::ComparativeSpace::lerp_clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1460,"record_offset":6580,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":9792,"source_line":25,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1460,"record_offset":6608,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1460,"record_offset":6396,"parent_record_offset":6328,"ancestor_record_offset":6328,"inlinee":26187,"source_line":226,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1460,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":6328,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1460,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":6328,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":1460,"record_offset":6440,"parent_record_offset":6396,"ancestor_record_offset":6328,"inlinee":26188,"source_line":3227,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1460,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":6328,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1460,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":6328,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01460.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_path_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_path_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
// 信息 ID: 06bb589a4ab8de5dfeb04537
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=16180; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=410, inline_record=9016; 不是独立调用边。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>; module=1471, inline_record=2960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
// 信息 ID: b77915ec4b7b47a7fa98dc7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ebe90 -> 0x1408600 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x81118f -> 0x1408600 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2> @ 0x141a7e9 -> 0x1408600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl::closure$0
// 信息 ID: ece7be817d6764699340d6fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=6328; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl; module=1460, inline_record=6856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
