// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ivy/ivy_send_to_gpu_leaf.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 8917fdfa93ed0de86543cce06200523a128250c355b53c83e43716bf3b55bf4a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_send_to_gpu_leaf.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 458, 469
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf
// 记录形式: compiled_procedure; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":13656,"rva":6636080,"va":5375345200}
// 编译实例 RVAs: 0x654230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2716744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8b88bd6ff","size":5,"soft_fail":false,"target_va":5372630832,"va":5375347571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2596445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8a361d8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375346152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t77169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8712d0100","size":5,"soft_fail":false,"target_va":5375423344,"va":5375346170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t939880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e868570e00","size":5,"soft_fail":false,"target_va":5376286288,"va":5375346403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7479707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e89b217200","size":5,"soft_fail":false,"target_va":5382826368,"va":5375346656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d793b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7491125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8354e7200","size":5,"soft_fail":false,"target_va":5382837168,"va":5375346038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7d0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7506677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8f58a7200","size":5,"soft_fail":false,"target_va":5382852800,"va":5375346118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7962487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8777f7900","size":5,"soft_fail":false,"target_va":5383308144,"va":5375345652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27611420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e81c51a501","size":5,"soft_fail":false,"target_va":5402958528,"va":5375347103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_leaf.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28247093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e83504af01","size":5,"soft_fail":false,"target_va":5403593344,"va":5375346246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27631511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8979fa501","size":5,"soft_fail":false,"target_va":5402978688,"va":5375347172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e870e5ffff","size":5,"soft_fail":false,"target_va":5375340240,"va":5375347035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27634419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8f3aaa501","size":5,"soft_fail":false,"target_va":5402980128,"va":5375345704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e84aeeffff","size":5,"soft_fail":false,"target_va":5375340752,"va":5375345281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27635300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e864aea501","size":5,"soft_fail":false,"target_va":5402980656,"va":5375345351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1011037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e85d6d0f00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375345870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1014923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e88b7c0f00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375345632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1010888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375345200,"raw_hex":"e8c86c0f00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375345251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375345200,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5375346266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":19132,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":17160,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":17244,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":19896,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":17076,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":469,"record_offset":17452,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":17692,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":14705,"source_line":356,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage::all_leaves
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_leaf_storage.rs
//     evidence={"module":469,"record_offset":16508,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":25309,"source_line":79,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":15440,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":13852,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":14428,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":13656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":17384,"parent_record_offset":17292,"ancestor_record_offset":13656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":17320,"parent_record_offset":17292,"ancestor_record_offset":13656,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":17292,"parent_record_offset":17268,"ancestor_record_offset":13656,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":17004,"parent_record_offset":16976,"ancestor_record_offset":13656,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":15860,"parent_record_offset":15832,"ancestor_record_offset":13656,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":13656,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":15888,"parent_record_offset":15860,"ancestor_record_offset":13656,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":15800,"parent_record_offset":15768,"ancestor_record_offset":13656,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":15832,"parent_record_offset":15800,"ancestor_record_offset":13656,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":16004,"parent_record_offset":15832,"ancestor_record_offset":13656,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":13656,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":17520,"parent_record_offset":17480,"ancestor_record_offset":13656,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":16828,"parent_record_offset":16800,"ancestor_record_offset":13656,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":16888,"parent_record_offset":16856,"ancestor_record_offset":13656,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":469,"record_offset":17032,"parent_record_offset":17004,"ancestor_record_offset":13656,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":16976,"parent_record_offset":16508,"ancestor_record_offset":13656,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":469,"record_offset":15936,"parent_record_offset":15912,"ancestor_record_offset":13656,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":17348,"parent_record_offset":17320,"ancestor_record_offset":13656,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":469,"record_offset":14064,"parent_record_offset":13968,"ancestor_record_offset":13656,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":15692,"parent_record_offset":15664,"ancestor_record_offset":13656,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":15024,"parent_record_offset":14564,"ancestor_record_offset":13656,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":15740,"parent_record_offset":15560,"ancestor_record_offset":13656,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":14376,"parent_record_offset":14348,"ancestor_record_offset":13656,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":16068,"parent_record_offset":15768,"ancestor_record_offset":13656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":469,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":13656,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":13656,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":13656,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":14984,"parent_record_offset":14952,"ancestor_record_offset":13656,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":15960,"parent_record_offset":15936,"ancestor_record_offset":13656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":16468,"parent_record_offset":15520,"ancestor_record_offset":13656,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":17864,"parent_record_offset":17792,"ancestor_record_offset":13656,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":469,"record_offset":17604,"parent_record_offset":17576,"ancestor_record_offset":13656,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":469,"record_offset":17632,"parent_record_offset":17604,"ancestor_record_offset":13656,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":14000,"parent_record_offset":13968,"ancestor_record_offset":13656,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":17732,"parent_record_offset":17692,"ancestor_record_offset":13656,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":14028,"parent_record_offset":14000,"ancestor_record_offset":13656,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":469,"record_offset":17100,"parent_record_offset":17076,"ancestor_record_offset":13656,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":13656,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":15080,"parent_record_offset":15056,"ancestor_record_offset":13656,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":14728,"parent_record_offset":14620,"ancestor_record_offset":13656,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":13656,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":14460,"parent_record_offset":14428,"ancestor_record_offset":13656,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":14592,"parent_record_offset":14564,"ancestor_record_offset":13656,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":14564,"parent_record_offset":14532,"ancestor_record_offset":13656,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":13656,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":13656,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":13656,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":15056,"parent_record_offset":14532,"ancestor_record_offset":13656,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":14164,"parent_record_offset":14132,"ancestor_record_offset":13656,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":14324,"parent_record_offset":14164,"ancestor_record_offset":13656,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":14132,"parent_record_offset":14108,"ancestor_record_offset":13656,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":17548,"parent_record_offset":17520,"ancestor_record_offset":13656,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":13884,"parent_record_offset":13852,"ancestor_record_offset":13656,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":13656,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":13940,"parent_record_offset":13908,"ancestor_record_offset":13656,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":13908,"parent_record_offset":13884,"ancestor_record_offset":13656,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":14620,"parent_record_offset":14592,"ancestor_record_offset":13656,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":14868,"parent_record_offset":14620,"ancestor_record_offset":13656,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":14840,"parent_record_offset":14620,"ancestor_record_offset":13656,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":14672,"parent_record_offset":14620,"ancestor_record_offset":13656,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":17908,"parent_record_offset":17692,"ancestor_record_offset":13656,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":17932,"parent_record_offset":17908,"ancestor_record_offset":13656,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":469,"record_offset":17480,"parent_record_offset":17452,"ancestor_record_offset":13656,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":17760,"parent_record_offset":17692,"ancestor_record_offset":13656,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":17792,"parent_record_offset":17760,"ancestor_record_offset":13656,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":469,"record_offset":16588,"parent_record_offset":16560,"ancestor_record_offset":13656,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":16736,"parent_record_offset":16644,"ancestor_record_offset":13656,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":16616,"parent_record_offset":16588,"ancestor_record_offset":13656,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":16708,"parent_record_offset":16644,"ancestor_record_offset":13656,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":16680,"parent_record_offset":16644,"ancestor_record_offset":13656,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":16644,"parent_record_offset":16616,"ancestor_record_offset":13656,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":469,"record_offset":16800,"parent_record_offset":16736,"ancestor_record_offset":13656,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":469,"record_offset":16856,"parent_record_offset":16800,"ancestor_record_offset":13656,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":15480,"parent_record_offset":15440,"ancestor_record_offset":13656,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":469,"record_offset":15604,"parent_record_offset":15560,"ancestor_record_offset":13656,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":15768,"parent_record_offset":15560,"ancestor_record_offset":13656,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":16040,"parent_record_offset":15768,"ancestor_record_offset":13656,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":469,"record_offset":17836,"parent_record_offset":17792,"ancestor_record_offset":13656,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":16560,"parent_record_offset":16532,"ancestor_record_offset":13656,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":13656,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":469,"record_offset":15632,"parent_record_offset":15604,"ancestor_record_offset":13656,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":469,"record_offset":15664,"parent_record_offset":15632,"ancestor_record_offset":13656,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":15560,"parent_record_offset":15520,"ancestor_record_offset":13656,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":15520,"parent_record_offset":15480,"ancestor_record_offset":13656,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669C
// 首条位置: {"module":458,"inlinee":26268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":458,"record_offset":21380,"parent_record_offset":20232,"ancestor_record_offset":20232,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":458,"record_offset":20316,"parent_record_offset":20232,"ancestor_record_offset":20232,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":20288,"parent_record_offset":20232,"ancestor_record_offset":20232,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":20232,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":20600,"parent_record_offset":20576,"ancestor_record_offset":20232,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":20684,"parent_record_offset":20660,"ancestor_record_offset":20232,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":458,"record_offset":20624,"parent_record_offset":20600,"ancestor_record_offset":20232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":20232,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":21044,"parent_record_offset":21020,"ancestor_record_offset":20232,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":20936,"parent_record_offset":20912,"ancestor_record_offset":20232,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":458,"record_offset":21156,"parent_record_offset":21132,"ancestor_record_offset":20232,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":458,"record_offset":21108,"parent_record_offset":21084,"ancestor_record_offset":20232,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":20376,"parent_record_offset":20352,"ancestor_record_offset":20232,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":20232,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":21084,"parent_record_offset":20864,"ancestor_record_offset":20232,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":20428,"parent_record_offset":20400,"ancestor_record_offset":20232,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20996,"parent_record_offset":20972,"ancestor_record_offset":20232,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20972,"parent_record_offset":20864,"ancestor_record_offset":20232,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20912,"parent_record_offset":20888,"ancestor_record_offset":20232,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":21292,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":20232,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20480,"parent_record_offset":20456,"ancestor_record_offset":20232,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20456,"parent_record_offset":20400,"ancestor_record_offset":20232,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20504,"parent_record_offset":20480,"ancestor_record_offset":20232,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":20232,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":20232,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":20832,"parent_record_offset":20716,"ancestor_record_offset":20232,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":20716,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20548,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":20864,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":20576,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":20660,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":21200,"parent_record_offset":20504,"ancestor_record_offset":20232,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":20352,"parent_record_offset":20316,"ancestor_record_offset":20232,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":21348,"parent_record_offset":20316,"ancestor_record_offset":20232,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x669B
// 首条位置: {"module":458,"inlinee":26267}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":458,"record_offset":19896,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":458,"record_offset":18832,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":458,"record_offset":18804,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":458,"record_offset":19648,"parent_record_offset":19624,"ancestor_record_offset":18748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":18748,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":458,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":18748,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":458,"record_offset":19140,"parent_record_offset":19116,"ancestor_record_offset":18748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":458,"record_offset":19312,"parent_record_offset":19288,"ancestor_record_offset":18748,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":19560,"parent_record_offset":19536,"ancestor_record_offset":18748,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":458,"record_offset":19452,"parent_record_offset":19428,"ancestor_record_offset":18748,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":458,"record_offset":19672,"parent_record_offset":19648,"ancestor_record_offset":18748,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":458,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":18748,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18748,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":18748,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":19600,"parent_record_offset":19380,"ancestor_record_offset":18748,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":458,"record_offset":18944,"parent_record_offset":18916,"ancestor_record_offset":18748,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19512,"parent_record_offset":19488,"ancestor_record_offset":18748,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19488,"parent_record_offset":19380,"ancestor_record_offset":18748,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19428,"parent_record_offset":19404,"ancestor_record_offset":18748,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19808,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19404,"parent_record_offset":19380,"ancestor_record_offset":18748,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":18748,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":18972,"parent_record_offset":18916,"ancestor_record_offset":18748,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":18748,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":19260,"parent_record_offset":19232,"ancestor_record_offset":18748,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":19288,"parent_record_offset":19260,"ancestor_record_offset":18748,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":19348,"parent_record_offset":19232,"ancestor_record_offset":18748,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":458,"record_offset":19232,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19064,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":458,"record_offset":19380,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":19092,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":19176,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":458,"record_offset":19716,"parent_record_offset":19020,"ancestor_record_offset":18748,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":458,"record_offset":18868,"parent_record_offset":18832,"ancestor_record_offset":18748,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00458.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":458,"record_offset":19864,"parent_record_offset":18832,"ancestor_record_offset":18748,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00458.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_send_to_gpu_leaf.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo,core::iter::adapters::take::Take<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo", "core::iter::adapters::take::Take<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_send_to_gpu_leaf.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_send_to_gpu_leaf.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf
// 信息 ID: 7ea1a4a9550997d101a1756e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8f7b0b3e8204f045f1a1d38: void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> @ 0x62a02c -> 0x654230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > @ 0xe421de -> 0x654230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure$0
// 信息 ID: a1197b9a6467186681dcb7e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> > >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> > >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$0>; module=458, inline_record=20232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure$1
// 信息 ID: 266cb69a537e4a5a38fbd4a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> > >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo,core::iter::adapters::take::Take<core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::resources::ivy::ivy_leaf_storage::IvyLeaf> > >,country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::closure_env$1>; module=458, inline_record=18748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
