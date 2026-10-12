// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/allocator.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c52708b68e3f32adeb9ccd8694d87736eef72e53af21abe844a9ce49a3d6927b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: allocator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 555, 570, 1593, 1598, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":1264,"rva":7514864,"va":5376223984}
// 编译实例 RVAs: 0x72aaf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27369259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376223984,"raw_hex":"e82b9fa101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376224096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t173080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376223984,"raw_hex":"e818a40200","size":5,"soft_fail":false,"target_va":5376397104,"va":5376224019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":555,"record_offset":1396,"parent_record_offset":1264,"ancestor_record_offset":1264,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":555,"record_offset":1428,"parent_record_offset":1264,"ancestor_record_offset":1264,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00555.jsonl.gz"}
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
// 首条位置: {"module":555,"record_offset":1460,"rva":7515072,"va":5376224192}
// 编译实例 RVAs: 0x72abc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3474724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8dcfacaff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376224415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3475089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e86ff9caff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376224796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c960
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8c71a0000","size":5,"soft_fail":false,"target_va":5376231776,"va":5376224916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117dd50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: allocator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10825711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8ef2fa500","size":5,"soft_fail":false,"target_va":5387050320,"va":5376224604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26753061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e825389801","size":5,"soft_fail":false,"target_va":5402978144,"va":5376225078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<gpu_allocator::vulkan::Allocation,enum2$<gpu_allocator::result::AllocationError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8bbfaffff","size":5,"soft_fail":false,"target_va":5376223776,"va":5376225120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t175832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8d8ae0200","size":5,"soft_fail":false,"target_va":5376400080,"va":5376224243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::generate_report
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t180879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e88fc20200","size":5,"soft_fail":false,"target_va":5376405152,"va":5376224268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1129013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376224192,"raw_hex":"e8353a1100","size":5,"soft_fail":false,"target_va":5377353776,"va":5376224758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":2596,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":555,"record_offset":2736,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":555,"record_offset":2764,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":1676,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":2820,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":555,"record_offset":1620,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":555,"record_offset":2792,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":555,"record_offset":2652,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1460,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1460,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":1460,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":2016,"parent_record_offset":1920,"ancestor_record_offset":1460,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":555,"record_offset":2960,"parent_record_offset":2868,"ancestor_record_offset":1460,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":1460,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":1920,"parent_record_offset":1888,"ancestor_record_offset":1460,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":1460,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":2136,"parent_record_offset":1888,"ancestor_record_offset":1460,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":555,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":1460,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":555,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1460,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":1460,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":1824,"parent_record_offset":1784,"ancestor_record_offset":1460,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":1856,"parent_record_offset":1824,"ancestor_record_offset":1460,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":1712,"parent_record_offset":1676,"ancestor_record_offset":1460,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":555,"record_offset":2404,"parent_record_offset":2372,"ancestor_record_offset":1460,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":555,"record_offset":2492,"parent_record_offset":2404,"ancestor_record_offset":1460,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":555,"record_offset":2176,"parent_record_offset":1784,"ancestor_record_offset":1460,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":555,"record_offset":1748,"parent_record_offset":1712,"ancestor_record_offset":1460,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":555,"record_offset":1784,"parent_record_offset":1748,"ancestor_record_offset":1460,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":555,"record_offset":1960,"parent_record_offset":1920,"ancestor_record_offset":1460,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":555,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1460,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":555,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":1460,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":555,"record_offset":2308,"parent_record_offset":2272,"ancestor_record_offset":1460,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":1460,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":555,"record_offset":2372,"parent_record_offset":2308,"ancestor_record_offset":1460,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":555,"record_offset":2924,"parent_record_offset":2896,"ancestor_record_offset":1460,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":1460,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":555,"record_offset":2532,"parent_record_offset":2272,"ancestor_record_offset":1460,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1460,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":555,"record_offset":2444,"parent_record_offset":2404,"ancestor_record_offset":1460,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":555,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1460,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":555,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":1460,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":1460,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":555,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":1460,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::allocator::impl$0::allocate::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/allocator.rs
//     evidence={"module":555,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1460,"inlinee":30563,"source_line":48,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::impl$0::allocate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7763
// 首条位置: {"module":555,"inlinee":30563}
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
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111E0
// 首条位置: {"module":1598,"inlinee":70112}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$63::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1598,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4214,"source_line":1565,"archive":"_pdb/A/modules/01598.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::free
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7767
// 首条位置: {"module":570,"inlinee":30567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::rename_allocation
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7768
// 首条位置: {"module":570,"inlinee":30568}
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
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":3268,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
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
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":3392,"rva":7516352,"va":5376225472}
// 编译实例 RVAs: 0x72b0c0
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
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":3536,"rva":7516368,"va":5376225488}
// 编译实例 RVAs: 0x72b0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":555,"record_offset":3684,"parent_record_offset":3536,"ancestor_record_offset":3536,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00555.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":3712,"rva":7516400,"va":5376225520}
// 编译实例 RVAs: 0x72b0f0
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
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":555,"record_offset":3852,"rva":7516416,"va":5376225536}
// 编译实例 RVAs: 0x72b100
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 24
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::allocate
// 信息 ID: bd5e9cdab2a0e568887fcc55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x26b105 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x26c330 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer_impl @ 0x738eb8 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xe9e3f5 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe9f620 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x11489d0 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x118d214 -> 0x72abc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::free
// 信息 ID: 9c3c8a849c5b04fac23cb0c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket; module=570, inline_record=61976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::inner
// 信息 ID: 7897c5dcc06e9d916cd50c8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::new
// 信息 ID: ca390d985b09da30292145a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73e20a -> 0x72aaf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_allocator_report
// 信息 ID: cb42f150d045d9ab486b8b0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0xe9a924 -> 0x72b0d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_device_local_bytes
// 信息 ID: 5cf86524ef3c3e64c2f8c9c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x246270 -> 0x72b0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0xe9a903 -> 0x72b0f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_system_bytes
// 信息 ID: 3cf52a564fdaf3b71c75592f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x24627b -> 0x72b100 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0xe9a90e -> 0x72b100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::peak_total_allocated_bytes
// 信息 ID: b86457283335587fcfe82172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_global_allocator<country_core::meta_commander::commander_update::closure_env$3,tuple$<u64,u64,u64,enum2$<core::option::Option<gpu_allocator::allocator::AllocatorReport> > > > @ 0xe9a919 -> 0x72b0c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::GpuAllocator::rename_allocation
// 信息 ID: 9f88908206d0f2eba091e692
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name; module=570, inline_record=13524; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::set_texture_debug_name; module=570, inline_record=79588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::impl$0::allocate::closure$0
// 信息 ID: 7b5926a7f317532c04754235
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::allocator::impl$0::allocate", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::allocator::GpuAllocator::allocate; module=555, inline_record=2336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::allocator::impl$0::allocate::closure$1
// 信息 ID: a4de428a2221ccdc3ef65680
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::allocator::impl$0::allocate", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::allocator::impl$0::allocate::closure_env$1 <- core::slice::sort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; module=1598, inline_record=1924; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; module=1598, inline_record=2008; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; module=1598, inline_record=2092; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; module=1598, inline_record=3500; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> > >; module=1598, inline_record=3796; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1598, inline_record=12876; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<str$>,u64>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<str$>,u64>,rhapsody::raw::allocator::impl$0::allocate::closure_env$1> >; module=1598, inline_record=13136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
