// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ivy/ivy_send_to_gpu_segments.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e50689bf4e6edf1259178ec5592eab9ae4e5a6d04ce5b2b9247d903677f86c07
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_send_to_gpu_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 406, 469, 1308
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
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
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":469,"record_offset":20816,"rva":6638992,"va":5375348112}
// 编译实例 RVAs: 0x654d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2718661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e83b84d6ff","size":5,"soft_fail":false,"target_va":5372630832,"va":5375349488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2599497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8b755d8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375349204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t74416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8b0220100","size":5,"soft_fail":false,"target_va":5375423344,"va":5375348923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t937134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8ae4c0e00","size":5,"soft_fail":false,"target_va":5376286288,"va":5375349149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d85a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7539167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8df097300","size":5,"soft_fail":false,"target_va":5382888080,"va":5375348908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7959575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e817747900","size":5,"soft_fail":false,"target_va":5383308144,"va":5375348564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27609238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e89648a501","size":5,"soft_fail":false,"target_va":5402958528,"va":5375349285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ivy_send_to_gpu_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28244340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e874f9ae01","size":5,"soft_fail":false,"target_va":5403593344,"va":5375348999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27629381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e84597a501","size":5,"soft_fail":false,"target_va":5402978688,"va":5375349302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8e4dcffff","size":5,"soft_fail":false,"target_va":5375340240,"va":5375349223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27631510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8969fa501","size":5,"soft_fail":false,"target_va":5402980128,"va":5375348613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e8ebe2ffff","size":5,"soft_fail":false,"target_va":5375340752,"va":5375348192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27632389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e805a3a501","size":5,"soft_fail":false,"target_va":5402980656,"va":5375348262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1008128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e800620f00","size":5,"soft_fail":false,"target_va":5376356912,"va":5375348779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1012011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e82b710f00","size":5,"soft_fail":false,"target_va":5376360560,"va":5375348544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1007977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375348112,"raw_hex":"e869610f00","size":5,"soft_fail":false,"target_va":5376356144,"va":5375348162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375348112,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5375349019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":469,"record_offset":24144,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":24756,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":24976,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":469,"record_offset":24240,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":24480,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":14705,"source_line":356,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":469,"record_offset":23668,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":22370,"source_line":163,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":22600,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":21012,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":21588,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":20816,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":469,"record_offset":24908,"parent_record_offset":24808,"ancestor_record_offset":20816,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":24840,"parent_record_offset":24808,"ancestor_record_offset":20816,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":469,"record_offset":24808,"parent_record_offset":24780,"ancestor_record_offset":20816,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":469,"record_offset":24172,"parent_record_offset":24144,"ancestor_record_offset":20816,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":23020,"parent_record_offset":22992,"ancestor_record_offset":20816,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":23072,"parent_record_offset":23048,"ancestor_record_offset":20816,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":23048,"parent_record_offset":23020,"ancestor_record_offset":20816,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":22960,"parent_record_offset":22928,"ancestor_record_offset":20816,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":22992,"parent_record_offset":22960,"ancestor_record_offset":20816,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":23164,"parent_record_offset":22992,"ancestor_record_offset":20816,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":469,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":20816,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":469,"record_offset":24308,"parent_record_offset":24268,"ancestor_record_offset":20816,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":20816,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":469,"record_offset":24052,"parent_record_offset":24020,"ancestor_record_offset":20816,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":469,"record_offset":24200,"parent_record_offset":24172,"ancestor_record_offset":20816,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":469,"record_offset":23096,"parent_record_offset":23072,"ancestor_record_offset":20816,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":469,"record_offset":24872,"parent_record_offset":24840,"ancestor_record_offset":20816,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":469,"record_offset":21224,"parent_record_offset":21128,"ancestor_record_offset":20816,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":22852,"parent_record_offset":22824,"ancestor_record_offset":20816,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":22184,"parent_record_offset":21724,"ancestor_record_offset":20816,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":469,"record_offset":22900,"parent_record_offset":22720,"ancestor_record_offset":20816,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":469,"record_offset":21536,"parent_record_offset":21508,"ancestor_record_offset":20816,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":23228,"parent_record_offset":22928,"ancestor_record_offset":20816,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":21960,"parent_record_offset":21936,"ancestor_record_offset":20816,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":21936,"parent_record_offset":21912,"ancestor_record_offset":20816,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":469,"record_offset":22144,"parent_record_offset":22112,"ancestor_record_offset":20816,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":469,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":20816,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":23628,"parent_record_offset":22680,"ancestor_record_offset":20816,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":469,"record_offset":24652,"parent_record_offset":24580,"ancestor_record_offset":20816,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":469,"record_offset":24392,"parent_record_offset":24364,"ancestor_record_offset":20816,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":469,"record_offset":24420,"parent_record_offset":24392,"ancestor_record_offset":20816,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":21160,"parent_record_offset":21128,"ancestor_record_offset":20816,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":469,"record_offset":24520,"parent_record_offset":24480,"ancestor_record_offset":20816,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":469,"record_offset":21188,"parent_record_offset":21160,"ancestor_record_offset":20816,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":20816,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":22240,"parent_record_offset":22216,"ancestor_record_offset":20816,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":469,"record_offset":21888,"parent_record_offset":21780,"ancestor_record_offset":20816,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":21128,"parent_record_offset":21100,"ancestor_record_offset":20816,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":469,"record_offset":21620,"parent_record_offset":21588,"ancestor_record_offset":20816,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":21752,"parent_record_offset":21724,"ancestor_record_offset":20816,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":469,"record_offset":21724,"parent_record_offset":21692,"ancestor_record_offset":20816,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":21668,"parent_record_offset":21644,"ancestor_record_offset":20816,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":21692,"parent_record_offset":21668,"ancestor_record_offset":20816,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":21644,"parent_record_offset":21620,"ancestor_record_offset":20816,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":469,"record_offset":22216,"parent_record_offset":21692,"ancestor_record_offset":20816,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21324,"parent_record_offset":21292,"ancestor_record_offset":20816,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21484,"parent_record_offset":21324,"ancestor_record_offset":20816,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":20816,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":24336,"parent_record_offset":24308,"ancestor_record_offset":20816,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21044,"parent_record_offset":21012,"ancestor_record_offset":20816,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20816,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21100,"parent_record_offset":21068,"ancestor_record_offset":20816,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":469,"record_offset":21068,"parent_record_offset":21044,"ancestor_record_offset":20816,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":21780,"parent_record_offset":21752,"ancestor_record_offset":20816,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":22028,"parent_record_offset":21780,"ancestor_record_offset":20816,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":22000,"parent_record_offset":21780,"ancestor_record_offset":20816,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":469,"record_offset":21832,"parent_record_offset":21780,"ancestor_record_offset":20816,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":24696,"parent_record_offset":24480,"ancestor_record_offset":20816,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":24720,"parent_record_offset":24696,"ancestor_record_offset":20816,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":469,"record_offset":24268,"parent_record_offset":24240,"ancestor_record_offset":20816,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":24548,"parent_record_offset":24480,"ancestor_record_offset":20816,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":469,"record_offset":24580,"parent_record_offset":24548,"ancestor_record_offset":20816,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":469,"record_offset":23752,"parent_record_offset":23724,"ancestor_record_offset":20816,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":23900,"parent_record_offset":23808,"ancestor_record_offset":20816,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":23780,"parent_record_offset":23752,"ancestor_record_offset":20816,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":23872,"parent_record_offset":23808,"ancestor_record_offset":20816,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":23844,"parent_record_offset":23808,"ancestor_record_offset":20816,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":469,"record_offset":23808,"parent_record_offset":23780,"ancestor_record_offset":20816,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":469,"record_offset":23964,"parent_record_offset":23900,"ancestor_record_offset":20816,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":469,"record_offset":24020,"parent_record_offset":23964,"ancestor_record_offset":20816,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":22640,"parent_record_offset":22600,"ancestor_record_offset":20816,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":469,"record_offset":22764,"parent_record_offset":22720,"ancestor_record_offset":20816,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":22928,"parent_record_offset":22720,"ancestor_record_offset":20816,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":469,"record_offset":23200,"parent_record_offset":22928,"ancestor_record_offset":20816,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":469,"record_offset":24624,"parent_record_offset":24580,"ancestor_record_offset":20816,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":23724,"parent_record_offset":23696,"ancestor_record_offset":20816,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":469,"record_offset":23696,"parent_record_offset":23668,"ancestor_record_offset":20816,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":469,"record_offset":22792,"parent_record_offset":22764,"ancestor_record_offset":20816,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":469,"record_offset":22824,"parent_record_offset":22792,"ancestor_record_offset":20816,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":22720,"parent_record_offset":22680,"ancestor_record_offset":20816,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":469,"record_offset":22680,"parent_record_offset":22640,"ancestor_record_offset":20816,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B35
// 首条位置: {"module":406,"inlinee":23349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":406,"record_offset":66840,"parent_record_offset":65652,"ancestor_record_offset":65652,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":406,"record_offset":66812,"parent_record_offset":65652,"ancestor_record_offset":65652,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":406,"record_offset":66752,"parent_record_offset":65652,"ancestor_record_offset":65652,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::Ivy::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":406,"record_offset":66784,"parent_record_offset":65652,"ancestor_record_offset":65652,"inlinee":23351,"source_line":50,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":406,"record_offset":65692,"parent_record_offset":65652,"ancestor_record_offset":65652,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":406,"record_offset":66504,"parent_record_offset":66480,"ancestor_record_offset":65652,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":406,"record_offset":65972,"parent_record_offset":65948,"ancestor_record_offset":65652,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":406,"record_offset":66056,"parent_record_offset":66032,"ancestor_record_offset":65652,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":406,"record_offset":65996,"parent_record_offset":65972,"ancestor_record_offset":65652,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":406,"record_offset":66864,"parent_record_offset":66840,"ancestor_record_offset":65652,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":406,"record_offset":66168,"parent_record_offset":66144,"ancestor_record_offset":65652,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":406,"record_offset":66416,"parent_record_offset":66392,"ancestor_record_offset":65652,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":406,"record_offset":66308,"parent_record_offset":66284,"ancestor_record_offset":65652,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":406,"record_offset":66528,"parent_record_offset":66504,"ancestor_record_offset":65652,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":406,"record_offset":66480,"parent_record_offset":66456,"ancestor_record_offset":65652,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":406,"record_offset":65748,"parent_record_offset":65724,"ancestor_record_offset":65652,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":406,"record_offset":65772,"parent_record_offset":65748,"ancestor_record_offset":65652,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":406,"record_offset":66456,"parent_record_offset":66236,"ancestor_record_offset":65652,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":406,"record_offset":65800,"parent_record_offset":65772,"ancestor_record_offset":65652,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66368,"parent_record_offset":66344,"ancestor_record_offset":65652,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66344,"parent_record_offset":66236,"ancestor_record_offset":65652,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66284,"parent_record_offset":66260,"ancestor_record_offset":65652,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66664,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66260,"parent_record_offset":66236,"ancestor_record_offset":65652,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":65852,"parent_record_offset":65828,"ancestor_record_offset":65652,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":65828,"parent_record_offset":65772,"ancestor_record_offset":65652,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":65876,"parent_record_offset":65852,"ancestor_record_offset":65652,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":66116,"parent_record_offset":66088,"ancestor_record_offset":65652,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":66144,"parent_record_offset":66116,"ancestor_record_offset":65652,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":66204,"parent_record_offset":66088,"ancestor_record_offset":65652,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":406,"record_offset":66088,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":65920,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":406,"record_offset":66236,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":65948,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":66032,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":406,"record_offset":66572,"parent_record_offset":65876,"ancestor_record_offset":65652,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":406,"record_offset":65724,"parent_record_offset":65692,"ancestor_record_offset":65652,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":406,"record_offset":66720,"parent_record_offset":65692,"ancestor_record_offset":65652,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00406.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B3A
// 首条位置: {"module":406,"inlinee":23354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":13808,"parent_record_offset":13776,"ancestor_record_offset":13776,"inlinee":21861,"source_line":2771,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":15856,"parent_record_offset":15368,"ancestor_record_offset":15368,"inlinee":23355,"source_line":112,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":13836,"parent_record_offset":13808,"ancestor_record_offset":13776,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":13872,"parent_record_offset":13836,"ancestor_record_offset":13776,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":13936,"parent_record_offset":13836,"ancestor_record_offset":13776,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1308,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":13776,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":13904,"parent_record_offset":13872,"ancestor_record_offset":13776,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1308,"record_offset":13984,"parent_record_offset":13960,"ancestor_record_offset":13776,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1308,"record_offset":16108,"parent_record_offset":16084,"ancestor_record_offset":16056,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1308,"record_offset":14196,"parent_record_offset":14068,"ancestor_record_offset":13776,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01308.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_send_to_gpu_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo", "core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_send_to_gpu_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_send_to_gpu_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 18; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments
// 信息 ID: 8b046b769744740d7138f893
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a6cc3567e30d18dfceb2432a: void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> @ 0x55e7bd -> 0x654d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)> > @ 0xe5348f -> 0x654d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0
// 信息 ID: 317c1fc865fed2243e23d5a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::ivy::ivy_storage::Ivy> >,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::ivy::ivy_storage::Ivy> >,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=406, inline_record=65652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure$1
// 信息 ID: a3a2b57ab1743dd1fcebc59e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1 <- core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=1308, inline_record=13776; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=1308, inline_record=15368; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<std::collections::hash::map::Values<usize,country_core::resources::ivy::ivy_storage::Ivy>,core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure$0::closure_env$1>,country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::closure_env$0>; module=1308, inline_record=16056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
