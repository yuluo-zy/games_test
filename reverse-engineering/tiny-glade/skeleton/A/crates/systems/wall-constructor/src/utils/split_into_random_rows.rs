// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/split_into_random_rows.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2b2cd0bbfb338d1ae827b564be9abc60ed3e83058acd95a797b46892c0effcb9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: split_into_random_rows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2725
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2725,"record_offset":3068,"rva":24643744,"va":5393352864}
// 编译实例 RVAs: 0x17808a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: split_into_random_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20604463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8d199c5fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393354170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: split_into_random_rows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10240322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e842419c00","size":5,"soft_fail":false,"target_va":5403594096,"va":5393353769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9564609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8c1f19100","size":5,"soft_fail":false,"target_va":5402918528,"va":5393353914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22045692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8049caffe","size":5,"soft_fail":false,"target_va":5371308432,"va":5393354119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9624384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e840db9200","size":5,"soft_fail":false,"target_va":5402978688,"va":5393354299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8bcf5ffff","size":5,"soft_fail":false,"target_va":5393351584,"va":5393354207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9626726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e866e49200","size":5,"soft_fail":false,"target_va":5402980128,"va":5393353397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e83bfbffff","size":5,"soft_fail":false,"target_va":5393351776,"va":5393352992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9627582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8bee79200","size":5,"soft_fail":false,"target_va":5402980656,"va":5393353069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16996653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8d3a6fcfe","size":5,"soft_fail":false,"target_va":5376356912,"va":5393353560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16992773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8fbb5fcfe","size":5,"soft_fail":false,"target_va":5376360560,"va":5393353328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16996829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e823a6fcfe","size":5,"soft_fail":false,"target_va":5376356144,"va":5393352968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11487929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e847b550ff","size":5,"soft_fail":false,"target_va":5381865424,"va":5393353348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16414276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393352864,"raw_hex":"e8bc8905ff","size":5,"soft_fail":false,"target_va":5376939616,"va":5393353887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2725,"record_offset":5924,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2725,"record_offset":6548,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2725,"record_offset":6328,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2725,"record_offset":6680,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$108::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":2725,"record_offset":6244,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17484,"source_line":57,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$218::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":2725,"record_offset":6156,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21889,"source_line":35,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2725,"record_offset":7136,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2725,"record_offset":5952,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":2725,"record_offset":6036,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":2725,"record_offset":6212,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":4876,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":3288,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":3864,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2725,"record_offset":7108,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2725,"record_offset":6300,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2725,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2725,"record_offset":7276,"parent_record_offset":7184,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":3068,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":7212,"parent_record_offset":7184,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":6384,"parent_record_offset":6352,"ancestor_record_offset":3068,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":3068,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2725,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2725,"record_offset":6592,"parent_record_offset":6548,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5304,"parent_record_offset":5276,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5216,"parent_record_offset":5188,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5420,"parent_record_offset":5248,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2725,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2725,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2725,"record_offset":7240,"parent_record_offset":7212,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2725,"record_offset":3500,"parent_record_offset":3404,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":7024,"parent_record_offset":6964,"ancestor_record_offset":3068,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":7056,"parent_record_offset":6964,"ancestor_record_offset":3068,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":3068,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":3068,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2725,"record_offset":6900,"parent_record_offset":6864,"ancestor_record_offset":3068,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2725,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2725,"record_offset":4460,"parent_record_offset":4000,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2725,"record_offset":5160,"parent_record_offset":4984,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2725,"record_offset":3812,"parent_record_offset":3784,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2725,"record_offset":5484,"parent_record_offset":5188,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2725,"record_offset":6000,"parent_record_offset":5976,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2725,"record_offset":6824,"parent_record_offset":6796,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2725,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2725,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2725,"record_offset":4420,"parent_record_offset":4388,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2725,"record_offset":5376,"parent_record_offset":5352,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2725,"record_offset":5884,"parent_record_offset":4948,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2725,"record_offset":3436,"parent_record_offset":3404,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2725,"record_offset":3464,"parent_record_offset":3436,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2725,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2725,"record_offset":6796,"parent_record_offset":6708,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2725,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2725,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2725,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2725,"record_offset":4164,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2725,"record_offset":3404,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2725,"record_offset":3896,"parent_record_offset":3864,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2725,"record_offset":4028,"parent_record_offset":4000,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2725,"record_offset":4000,"parent_record_offset":3968,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2725,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2725,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2725,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2725,"record_offset":4492,"parent_record_offset":3968,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3600,"parent_record_offset":3568,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3760,"parent_record_offset":3600,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3320,"parent_record_offset":3288,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2725,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2725,"record_offset":4056,"parent_record_offset":4028,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2725,"record_offset":4304,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2725,"record_offset":4276,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2725,"record_offset":4108,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":2725,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":3068,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2725,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":3068,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2725,"record_offset":5024,"parent_record_offset":4984,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":5188,"parent_record_offset":4984,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2725,"record_offset":5456,"parent_record_offset":5188,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2725,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2725,"record_offset":5084,"parent_record_offset":5052,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2725,"record_offset":4984,"parent_record_offset":4948,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2725,"record_offset":4948,"parent_record_offset":4912,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/split_into_random_rows.rs
//     evidence={"module":2725,"record_offset":6936,"parent_record_offset":6900,"ancestor_record_offset":3068,"inlinee":89669,"source_line":32,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::split_into_random_rows::split_into_random_rows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E45
// 首条位置: {"module":2725,"inlinee":89669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2725,"record_offset":6964,"parent_record_offset":6936,"ancestor_record_offset":6936,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":7024,"parent_record_offset":6964,"ancestor_record_offset":6936,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":7056,"parent_record_offset":6964,"ancestor_record_offset":6936,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02725.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2725,"record_offset":6992,"parent_record_offset":6964,"ancestor_record_offset":6936,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02725.jsonl.gz"}
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
// 完整结构及原始类型记录: split_into_random_rows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: split_into_random_rows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: split_into_random_rows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
// 信息 ID: d9bbf1df8b334abddce7d8fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdb297 -> 0x17808a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::split_into_random_rows::split_into_random_rows::closure$0
// 信息 ID: 8709ce61b2cae5245e057ba5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::split_into_random_rows::split_into_random_rows", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows; module=2725, inline_record=6936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
