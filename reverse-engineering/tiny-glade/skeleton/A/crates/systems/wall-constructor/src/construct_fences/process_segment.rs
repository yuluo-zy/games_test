// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_fences/process_segment.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e885c685f3f7071a76fcc3a89b5d8e90e355162a4f5ae4df05bb724f5d8e5a32
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1234, 1246, 1263, 1264, 1265, 2718
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.20.5"},{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1246,"record_offset":3068,"rva":13405616,"va":5382114736}
// 编译实例 RVAs: 0xcc8db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9366040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e8e81571ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382115747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbb910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e83a28ffff","size":5,"soft_fail":false,"target_va":5382060304,"va":5382115537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20863067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e85b583e01","size":5,"soft_fail":false,"target_va":5402978688,"va":5382115616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e8aef7ffff","size":5,"soft_fail":false,"target_va":5382113456,"va":5382115581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20864901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e8855f3e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382115222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e877fbffff","size":5,"soft_fail":false,"target_va":5382113648,"va":5382114804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20865770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e8ea623e01","size":5,"soft_fail":false,"target_va":5402980656,"va":5382114881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5758479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e8f121a8ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5382115386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5754598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e81a31a8ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5382115153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5758645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e84b21a8ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5382114784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-249754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382114736,"raw_hex":"e86630fcff","size":5,"soft_fail":false,"target_va":5381865424,"va":5382115173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":5988,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":6016,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":4928,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":3320,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":3908,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1246,"record_offset":5960,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":59866,"source_line":15,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":6168,"parent_record_offset":6068,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":6100,"parent_record_offset":6068,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":6068,"parent_record_offset":6040,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5284,"parent_record_offset":5252,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5456,"parent_record_offset":5284,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1246,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1246,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1246,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1246,"record_offset":3536,"parent_record_offset":3440,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1246,"record_offset":5148,"parent_record_offset":5120,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1246,"record_offset":4508,"parent_record_offset":4048,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1246,"record_offset":5196,"parent_record_offset":5024,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1246,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":5520,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1246,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1246,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1246,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1246,"record_offset":5920,"parent_record_offset":4992,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1246,"record_offset":3472,"parent_record_offset":3440,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1246,"record_offset":3500,"parent_record_offset":3472,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1246,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1246,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1246,"record_offset":4212,"parent_record_offset":4104,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1246,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1246,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1246,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1246,"record_offset":4048,"parent_record_offset":4016,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1246,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1246,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1246,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1246,"record_offset":4540,"parent_record_offset":4016,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3644,"parent_record_offset":3608,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3804,"parent_record_offset":3644,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3356,"parent_record_offset":3320,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1246,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1246,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1246,"record_offset":4352,"parent_record_offset":4104,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1246,"record_offset":4324,"parent_record_offset":4104,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1246,"record_offset":4156,"parent_record_offset":4104,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":4960,"parent_record_offset":4928,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1246,"record_offset":5060,"parent_record_offset":5024,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":5224,"parent_record_offset":5024,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1246,"record_offset":5492,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1246,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1246,"record_offset":5120,"parent_record_offset":5088,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1246,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1246,"record_offset":4992,"parent_record_offset":4960,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE953
// 首条位置: {"module":1234,"inlinee":59731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1296,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":840,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1234,"record_offset":768,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1234,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":712,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1332,"parent_record_offset":1296,"ancestor_record_offset":712,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":712,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":712,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1234,"record_offset":996,"parent_record_offset":960,"ancestor_record_offset":712,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1234,"record_offset":960,"parent_record_offset":916,"ancestor_record_offset":712,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":1116,"parent_record_offset":996,"ancestor_record_offset":712,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1234,"record_offset":1080,"parent_record_offset":1032,"ancestor_record_offset":712,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1234,"record_offset":1396,"parent_record_offset":1296,"ancestor_record_offset":712,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1234,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":712,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1234,"record_offset":1032,"parent_record_offset":996,"ancestor_record_offset":712,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1216,"parent_record_offset":916,"ancestor_record_offset":712,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1152,"parent_record_offset":916,"ancestor_record_offset":712,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1234,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":712,"inlinee":59733,"source_line":40,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1234,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":712,"inlinee":27397,"source_line":875,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1234,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":712,"inlinee":23220,"source_line":91,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE955
// 首条位置: {"module":1234,"inlinee":59733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1234,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":868,"inlinee":27397,"source_line":875,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":868,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":868,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1234,"record_offset":996,"parent_record_offset":960,"ancestor_record_offset":868,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1234,"record_offset":960,"parent_record_offset":916,"ancestor_record_offset":868,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":1116,"parent_record_offset":996,"ancestor_record_offset":868,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1234,"record_offset":1080,"parent_record_offset":1032,"ancestor_record_offset":868,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1234,"record_offset":1032,"parent_record_offset":996,"ancestor_record_offset":868,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1216,"parent_record_offset":916,"ancestor_record_offset":868,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1152,"parent_record_offset":916,"ancestor_record_offset":868,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1234,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":868,"inlinee":23220,"source_line":91,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1246,"record_offset":6240,"rva":13406672,"va":5382115792}
// 编译实例 RVAs: 0xcc91d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12304927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8e13d44ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382118474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10420567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8a9fe60ff","size":5,"soft_fail":false,"target_va":5371700720,"va":5382121282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9366349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8b31471ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382116040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9368339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8ed0c71ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382118046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5173460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e82c0fb1ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382117423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-198782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e882f7fcff","size":5,"soft_fail":false,"target_va":5381919744,"va":5382118521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e836bafeff","size":5,"soft_fail":false,"target_va":5382033008,"va":5382116405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-79833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e827c8feff","size":5,"soft_fail":false,"target_va":5382036864,"va":5382116692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceda30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t148530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e832440200","size":5,"soft_fail":false,"target_va":5382265392,"va":5382116857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21476990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e87eb64701","size":5,"soft_fail":false,"target_va":5403593360,"va":5382116365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21477354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8eab74701","size":5,"soft_fail":false,"target_va":5403593968,"va":5382116609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21475196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e87caf4701","size":5,"soft_fail":false,"target_va":5403594096,"va":5382118895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20857466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e87a423e01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382120673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-251161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8e72afcff","size":5,"soft_fail":false,"target_va":5381866320,"va":5382117476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4593565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e863e8b9ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382120680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20934202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e83a6e3f01","size":5,"soft_fail":false,"target_va":5403054752,"va":5382120545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20858888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e808483e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382120451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20859465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8494a3e01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382120658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20859714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8424b3e01","size":5,"soft_fail":false,"target_va":5402980304,"va":5382120585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20859833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8b94b3e01","size":5,"soft_fail":false,"target_va":5402980400,"va":5382120562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20859710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e83e4b3e01","size":5,"soft_fail":false,"target_va":5402980320,"va":5382120605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::render::Affine3Packed::from_cols
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-248311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e80936fcff","size":5,"soft_fail":false,"target_va":5381870160,"va":5382118466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_disc_vec2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2271128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e898a72200","size":5,"soft_fail":false,"target_va":5384388416,"va":5382117283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t2270630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8a6a52200","size":5,"soft_fail":false,"target_va":5384387936,"va":5382117301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::usize_less_than
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2268164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8049c2200","size":5,"soft_fail":false,"target_va":5384387792,"va":5382119623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11226909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e81d4fab00","size":5,"soft_fail":false,"target_va":5393342896,"va":5382115982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::max_extent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2304186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382115792,"raw_hex":"e8ba282300","size":5,"soft_fail":false,"target_va":5384420688,"va":5382116497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1246,"record_offset":7448,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":6656,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1246,"record_offset":12948,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1246,"record_offset":7252,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1246,"record_offset":13004,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::enumerate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1246,"record_offset":7196,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":16076,"source_line":976,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":11192,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1246,"record_offset":6992,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1246,"record_offset":7308,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":13920,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14596,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14412,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":13228,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14164,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":11892,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14076,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":12372,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":13692,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14384,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":13980,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1246,"record_offset":7076,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1246,"record_offset":12636,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":8036,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::choose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":13948,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":22380,"source_line":203,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":6600,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":23177,"source_line":144,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":8204,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":12920,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::place_fence_piece
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1246,"record_offset":12008,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":59870,"source_line":308,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1246,"record_offset":8564,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":59868,"source_line":131,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1246,"record_offset":8176,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1246,"record_offset":7900,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1246,"record_offset":7972,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":6240,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":6240,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":11380,"parent_record_offset":11356,"ancestor_record_offset":6240,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":6828,"parent_record_offset":6740,"ancestor_record_offset":6240,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1246,"record_offset":11356,"parent_record_offset":11264,"ancestor_record_offset":6240,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":6240,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":9160,"parent_record_offset":9128,"ancestor_record_offset":6240,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":6740,"parent_record_offset":6712,"ancestor_record_offset":6240,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":6240,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":6948,"parent_record_offset":6712,"ancestor_record_offset":6240,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":6240,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1246,"record_offset":7544,"parent_record_offset":7512,"ancestor_record_offset":6240,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1246,"record_offset":7512,"parent_record_offset":7480,"ancestor_record_offset":6240,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1246,"record_offset":7480,"parent_record_offset":7448,"ancestor_record_offset":6240,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":9128,"parent_record_offset":9068,"ancestor_record_offset":6240,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":9068,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":6240,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1246,"record_offset":6772,"parent_record_offset":6740,"ancestor_record_offset":6240,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1246,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":6240,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1246,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":6240,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1246,"record_offset":8000,"parent_record_offset":7972,"ancestor_record_offset":6240,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1246,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6240,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1246,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":6240,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::Enumerate::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1246,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6240,"inlinee":16078,"source_line":23,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1246,"record_offset":13028,"parent_record_offset":13004,"ancestor_record_offset":6240,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1246,"record_offset":8500,"parent_record_offset":8340,"ancestor_record_offset":6240,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1246,"record_offset":11320,"parent_record_offset":11292,"ancestor_record_offset":6240,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":8088,"parent_record_offset":8036,"ancestor_record_offset":6240,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":8144,"parent_record_offset":8036,"ancestor_record_offset":6240,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":13720,"parent_record_offset":13692,"ancestor_record_offset":6240,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":6240,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":6240,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1246,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":6240,"inlinee":27288,"source_line":454,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1246,"record_offset":7828,"parent_record_offset":7544,"ancestor_record_offset":6240,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":6240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1246,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6240,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1246,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":6240,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1246,"record_offset":7156,"parent_record_offset":7124,"ancestor_record_offset":6240,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":6240,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":9192,"parent_record_offset":9068,"ancestor_record_offset":6240,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1246,"record_offset":7928,"parent_record_offset":7900,"ancestor_record_offset":6240,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1246,"record_offset":7400,"parent_record_offset":7336,"ancestor_record_offset":6240,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$8::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1246,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":6240,"inlinee":20602,"source_line":583,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1246,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6240,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1246,"record_offset":8440,"parent_record_offset":8376,"ancestor_record_offset":6240,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1246,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6240,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1246,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":6240,"inlinee":25313,"source_line":238,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9652,"parent_record_offset":9620,"ancestor_record_offset":6240,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":6240,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":6240,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9564,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9476,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9020,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8916,"parent_record_offset":8884,"ancestor_record_offset":6240,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8884,"parent_record_offset":8852,"ancestor_record_offset":6240,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":6240,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8824,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9328,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":14436,"parent_record_offset":14412,"ancestor_record_offset":6240,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9240,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::vec2::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.20.5/src/vec.rs
//     evidence={"module":1246,"record_offset":8656,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":25311,"source_line":576,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1246,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":6240,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1246,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":6240,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":9420,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":22397,"source_line":270,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":6240,"inlinee":22398,"source_line":275,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::from_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":6240,"inlinee":23179,"source_line":66,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":9784,"parent_record_offset":8564,"ancestor_record_offset":6240,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":8948,"parent_record_offset":8916,"ancestor_record_offset":6240,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1246,"record_offset":12336,"parent_record_offset":12008,"ancestor_record_offset":6240,"inlinee":59872,"source_line":47,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9DC
// 首条位置: {"module":1246,"inlinee":59868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":9068,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9564,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9476,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9020,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8824,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9328,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":9240,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] glam::vec2::impl$25::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.20.5/src/vec.rs
//     evidence={"module":1246,"record_offset":8656,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":25311,"source_line":576,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":9420,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":22397,"source_line":270,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":9784,"parent_record_offset":8564,"ancestor_record_offset":8564,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1246,"record_offset":9160,"parent_record_offset":9128,"ancestor_record_offset":8564,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1246,"record_offset":9128,"parent_record_offset":9068,"ancestor_record_offset":8564,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":8564,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":9360,"parent_record_offset":9328,"ancestor_record_offset":8564,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":9504,"parent_record_offset":9476,"ancestor_record_offset":8564,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1246,"record_offset":9192,"parent_record_offset":9068,"ancestor_record_offset":8564,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::core::scalar::vector::impl$9::mul_scalar
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.20.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.20.5/src/core/scalar/vector.rs
//     evidence={"module":1246,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":8564,"inlinee":25313,"source_line":238,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9652,"parent_record_offset":9620,"ancestor_record_offset":8564,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9620,"parent_record_offset":9588,"ancestor_record_offset":8564,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1246,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":8564,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8916,"parent_record_offset":8884,"ancestor_record_offset":8564,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8884,"parent_record_offset":8852,"ancestor_record_offset":8564,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":8852,"parent_record_offset":8824,"ancestor_record_offset":8564,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":1246,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":8564,"inlinee":22398,"source_line":275,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1246,"record_offset":8948,"parent_record_offset":8916,"ancestor_record_offset":8564,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
// 记录形式: compiled_procedure; instances=1
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2718,"record_offset":260,"rva":24633776,"va":5393342896}
// 编译实例 RVAs: 0x177e1b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20593282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e87ec5c5fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5393342973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20594061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e873c2c5fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393343768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cedef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11076480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e880fc56ff","size":5,"soft_fail":false,"target_va":5382266608,"va":5393343083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: process_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10251047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e8276b9c00","size":5,"soft_fail":false,"target_va":5403594000,"va":5393342948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9634447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e88f029300","size":5,"soft_fail":false,"target_va":5402978144,"va":5393343692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-22038435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e85db8affe","size":5,"soft_fail":false,"target_va":5371305056,"va":5393343486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9711113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e8092e9400","size":5,"soft_fail":false,"target_va":5403054752,"va":5393343634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9635746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393342896,"raw_hex":"e8a2079300","size":5,"soft_fail":false,"target_va":5402979344,"va":5393343593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":500,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2718,"record_offset":648,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2718,"record_offset":1260,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2718,"record_offset":1032,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2718,"record_offset":732,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2718,"record_offset":1956,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2718,"record_offset":1460,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2718,"record_offset":1384,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":2718,"record_offset":788,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2718,"record_offset":472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_direct] utils::cos_angle_between
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2718,"record_offset":1636,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59882,"source_line":447,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2718,"record_offset":2108,"parent_record_offset":2008,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2718,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2718,"record_offset":1320,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2718,"record_offset":2008,"parent_record_offset":1980,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2718,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":260,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2718,"record_offset":1296,"parent_record_offset":1260,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2718,"record_offset":696,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2718,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2718,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":260,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2718,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2718,"record_offset":852,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2718,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2718,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2718,"record_offset":968,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2718,"record_offset":2072,"parent_record_offset":2040,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2718,"record_offset":1488,"parent_record_offset":1460,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2718,"record_offset":1172,"parent_record_offset":1140,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2718,"record_offset":1140,"parent_record_offset":1108,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2718,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2718,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2718,"record_offset":1352,"parent_record_offset":1260,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2718,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2718,"record_offset":1688,"parent_record_offset":1636,"ancestor_record_offset":260,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":2718,"record_offset":1068,"parent_record_offset":1032,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":2718,"record_offset":816,"parent_record_offset":788,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2718,"record_offset":1800,"parent_record_offset":1636,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":2718,"record_offset":1204,"parent_record_offset":1172,"ancestor_record_offset":260,"inlinee":59864,"source_line":287,"archive":"_pdb/A/modules/02718.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9D8
// 首条位置: {"module":1246,"inlinee":59864}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_fences::process_segment::place_fence_piece
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9DE
// 首条位置: {"module":1246,"inlinee":59870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1246,"record_offset":12052,"parent_record_offset":12008,"ancestor_record_offset":12008,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::construct_fences::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/mod.rs
//     evidence={"module":1246,"record_offset":12336,"parent_record_offset":12008,"ancestor_record_offset":12008,"inlinee":59872,"source_line":47,"archive":"_pdb/A/modules/01246.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1246,"record_offset":12088,"parent_record_offset":12052,"ancestor_record_offset":12008,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01246.jsonl.gz"}
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
// 完整结构及原始类型记录: process_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_fences::process_segment::process_segment::FencePost
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_wall_constructor::construct_fences::process_segment::process_segment::FencePost,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_wall_constructor::construct_fences::process_segment::process_segment::FencePost", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: process_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: process_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
// 信息 ID: c618d56ca1a3e744daccb1f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 <- utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1234, inline_record=712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
// 信息 ID: 387cd2260849b4bde1d9e8cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::emit_fences", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >; module=1234, inline_record=868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: 33527514c39762d45fb0c9c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x4e90cf -> 0xcc8db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::place_fence_piece
// 信息 ID: 1407d726e7204521c56ed5a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1246, inline_record=12008; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1246, inline_record=14908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment::closure$0
// 信息 ID: 70e1b7780859b7c09dfed2c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::process_segment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1246, inline_record=8564; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>; module=1246, inline_record=9944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// 信息 ID: 45968ebdb6c247f1f7e073cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::process_segment -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> > @ 0xcbbb7a -> 0xcc91d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle
// 信息 ID: 275f3b1060ab99064c22e3c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0xcc928e -> 0x177e1b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure$0
// 信息 ID: 02c99cb420593a036d0d0b8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0 <- itertools::tuple_impl::impl$25::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$25::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1265, inline_record=18168; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$25::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1265, inline_record=18688; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$25::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1265, inline_record=19124; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$25::collect_from_iter_no_buf<ref$<glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<ref$<glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle::closure_env$0> > > >; module=1265, inline_record=19356; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=2718, inline_record=1204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
