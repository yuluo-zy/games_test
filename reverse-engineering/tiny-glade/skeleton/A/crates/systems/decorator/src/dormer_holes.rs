// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/dormer_holes.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9ee2a10b2f9bcef6e4251d5f8c3a00a5c5fa9e18ac9be7349fc22bc7e440c73d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dormer_holes.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3852
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::dormer_holes
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3852,"record_offset":4184,"rva":29670416,"va":5398379536}
// 编译实例 RVAs: 0x1c4bc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25754782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8620377fe","size":5,"soft_fail":false,"target_va":5372630832,"va":5398385609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25630642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e84ee878fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398380333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25634185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e877da78fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398383892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23011558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e81adfa0fe","size":5,"soft_fail":false,"target_va":5375372576,"va":5398384129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22097950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8e2cfaefe","size":5,"soft_fail":false,"target_va":5376286288,"va":5398384233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4841841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e88f1eb6ff","size":5,"soft_fail":false,"target_va":5393540640,"va":5398382476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417af820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4839257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8a728b6ff","size":5,"soft_fail":false,"target_va":5393545248,"va":5398384500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c3d380
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-63996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e80406ffff","size":5,"soft_fail":false,"target_va":5398320000,"va":5398383991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5209272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8b87c4f00","size":5,"soft_fail":false,"target_va":5403593344,"va":5398384067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5210859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8eb824f00","size":5,"soft_fail":false,"target_va":5403593392,"va":5398382528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: dormer_holes.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5211112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8e8834f00","size":5,"soft_fail":false,"target_va":5403594096,"va":5398382979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4593154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e802164600","size":5,"soft_fail":false,"target_va":5402978144,"va":5398384985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27078270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e882d162fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5398383321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4593654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8f6174600","size":5,"soft_fail":false,"target_va":5402978688,"va":5398385029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5133520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8d0544e00","size":5,"soft_fail":false,"target_va":5403518608,"va":5398385083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4595425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8e11e4600","size":5,"soft_fail":false,"target_va":5402979344,"va":5398383914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8bbe5ffff","size":5,"soft_fail":false,"target_va":5398378160,"va":5398384880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4600019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8d3304600","size":5,"soft_fail":false,"target_va":5402980128,"va":5398380104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8cbfaffff","size":5,"soft_fail":false,"target_va":5398378352,"va":5398379680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4600894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e83e344600","size":5,"soft_fail":false,"target_va":5402980656,"va":5398379757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::dummy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23559308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8748398fe","size":5,"soft_fail":false,"target_va":5374821008,"va":5398380311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21030063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8511bbffe","size":5,"soft_fail":false,"target_va":5377353776,"va":5398383834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22023363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e83df3affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398380270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22019477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e86b02b0fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398380032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22023522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e89ef2affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398379661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16090249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e8777b0aff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398380052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::aabb
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20419233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398379536,"raw_hex":"e85f6dc8fe","size":5,"soft_fail":false,"target_va":5377963712,"va":5398382940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5398379536,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5398384098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":7052,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":10272,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":8848,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3852,"record_offset":10328,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3852,"record_offset":8188,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3852,"record_offset":12520,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":10976,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3852,"record_offset":13376,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3852,"record_offset":8960,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3852,"record_offset":11268,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::InstanceBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3852,"record_offset":11508,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":14705,"source_line":356,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":5984,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":4376,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":4964,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3852,"record_offset":7828,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":89748,"source_line":256,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::dormer_holes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":3852,"record_offset":10460,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":111823,"source_line":41,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::get_cells_from_bbx
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":3852,"record_offset":9024,"parent_record_offset":4184,"ancestor_record_offset":4184,"inlinee":111820,"source_line":120,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":7232,"parent_record_offset":7208,"ancestor_record_offset":4184,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":4184,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":4184,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":4184,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":10200,"parent_record_offset":10100,"ancestor_record_offset":4184,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":10132,"parent_record_offset":10100,"ancestor_record_offset":4184,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":4184,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":7180,"parent_record_offset":7152,"ancestor_record_offset":4184,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":7152,"parent_record_offset":7124,"ancestor_record_offset":4184,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":7328,"parent_record_offset":7152,"ancestor_record_offset":4184,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":4184,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9960,"parent_record_offset":9924,"ancestor_record_offset":4184,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":7364,"parent_record_offset":7084,"ancestor_record_offset":4184,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9624,"parent_record_offset":9024,"ancestor_record_offset":4184,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9924,"parent_record_offset":9024,"ancestor_record_offset":4184,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":7124,"parent_record_offset":7084,"ancestor_record_offset":4184,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":10504,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":10600,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":4184,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":11988,"parent_record_offset":11956,"ancestor_record_offset":4184,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3852,"record_offset":11000,"parent_record_offset":10976,"ancestor_record_offset":4184,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3852,"record_offset":11048,"parent_record_offset":11024,"ancestor_record_offset":4184,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":3852,"record_offset":7084,"parent_record_offset":7052,"ancestor_record_offset":4184,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":4184,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":4184,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6432,"parent_record_offset":6404,"ancestor_record_offset":4184,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6344,"parent_record_offset":6312,"ancestor_record_offset":4184,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6376,"parent_record_offset":6344,"ancestor_record_offset":4184,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6548,"parent_record_offset":6376,"ancestor_record_offset":4184,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3852,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":4184,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3852,"record_offset":10888,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3852,"record_offset":9748,"parent_record_offset":9680,"ancestor_record_offset":4184,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3852,"record_offset":11336,"parent_record_offset":11296,"ancestor_record_offset":4184,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3852,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":4184,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3852,"record_offset":8100,"parent_record_offset":8068,"ancestor_record_offset":4184,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3852,"record_offset":10916,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3852,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":4184,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3852,"record_offset":9652,"parent_record_offset":9024,"ancestor_record_offset":4184,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3852,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":4184,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3852,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":4184,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3852,"record_offset":10164,"parent_record_offset":10132,"ancestor_record_offset":4184,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3852,"record_offset":4592,"parent_record_offset":4496,"ancestor_record_offset":4184,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3852,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":4184,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":4184,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":4184,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3852,"record_offset":9716,"parent_record_offset":9680,"ancestor_record_offset":4184,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3852,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":4184,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3852,"record_offset":5564,"parent_record_offset":5104,"ancestor_record_offset":4184,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3852,"record_offset":6284,"parent_record_offset":6104,"ancestor_record_offset":4184,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3852,"record_offset":4912,"parent_record_offset":4884,"ancestor_record_offset":4184,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":6612,"parent_record_offset":6312,"ancestor_record_offset":4184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3852,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":4184,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3852,"record_offset":8680,"parent_record_offset":8656,"ancestor_record_offset":4184,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3852,"record_offset":8656,"parent_record_offset":8632,"ancestor_record_offset":4184,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3852,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":4184,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3852,"record_offset":10400,"parent_record_offset":10328,"ancestor_record_offset":4184,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3852,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":4184,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3852,"record_offset":5524,"parent_record_offset":5492,"ancestor_record_offset":4184,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":10424,"parent_record_offset":10400,"ancestor_record_offset":4184,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":7284,"parent_record_offset":7256,"ancestor_record_offset":4184,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":4184,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3852,"record_offset":7012,"parent_record_offset":6064,"ancestor_record_offset":4184,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3852,"record_offset":11612,"parent_record_offset":11576,"ancestor_record_offset":4184,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3852,"record_offset":11420,"parent_record_offset":11392,"ancestor_record_offset":4184,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3852,"record_offset":11448,"parent_record_offset":11420,"ancestor_record_offset":4184,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3852,"record_offset":4528,"parent_record_offset":4496,"ancestor_record_offset":4184,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3852,"record_offset":12644,"parent_record_offset":12604,"ancestor_record_offset":4184,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3852,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":4184,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":4184,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":4184,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":4184,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":4184,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":4184,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3852,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":4184,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3852,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":4184,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3852,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4184,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3852,"record_offset":5268,"parent_record_offset":5160,"ancestor_record_offset":4184,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3852,"record_offset":4496,"parent_record_offset":4468,"ancestor_record_offset":4184,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3852,"record_offset":5000,"parent_record_offset":4964,"ancestor_record_offset":4184,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3852,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":4184,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3852,"record_offset":5104,"parent_record_offset":5072,"ancestor_record_offset":4184,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3852,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4184,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3852,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4184,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3852,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4184,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3852,"record_offset":5596,"parent_record_offset":5072,"ancestor_record_offset":4184,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4700,"parent_record_offset":4664,"ancestor_record_offset":4184,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4860,"parent_record_offset":4700,"ancestor_record_offset":4184,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4664,"parent_record_offset":4636,"ancestor_record_offset":4184,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":4184,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4412,"parent_record_offset":4376,"ancestor_record_offset":4184,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":4184,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":4184,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3852,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":4184,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3852,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":4184,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3852,"record_offset":5408,"parent_record_offset":5160,"ancestor_record_offset":4184,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3852,"record_offset":5380,"parent_record_offset":5160,"ancestor_record_offset":4184,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3852,"record_offset":5212,"parent_record_offset":5160,"ancestor_record_offset":4184,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":4184,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":11656,"parent_record_offset":11508,"ancestor_record_offset":4184,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":4184,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":4184,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3852,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":4184,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3852,"record_offset":11544,"parent_record_offset":11508,"ancestor_record_offset":4184,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3852,"record_offset":11576,"parent_record_offset":11544,"ancestor_record_offset":4184,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":4184,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9208,"parent_record_offset":9088,"ancestor_record_offset":4184,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3852,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":4184,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3852,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":4184,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":8632,"parent_record_offset":8288,"ancestor_record_offset":4184,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":7984,"parent_record_offset":7924,"ancestor_record_offset":4184,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":4184,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":4184,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":7956,"parent_record_offset":7924,"ancestor_record_offset":4184,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":4184,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3852,"record_offset":8372,"parent_record_offset":8336,"ancestor_record_offset":4184,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3852,"record_offset":8408,"parent_record_offset":8372,"ancestor_record_offset":4184,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3852,"record_offset":8468,"parent_record_offset":8336,"ancestor_record_offset":4184,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3852,"record_offset":8336,"parent_record_offset":8288,"ancestor_record_offset":4184,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3852,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":4184,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3852,"record_offset":8012,"parent_record_offset":7984,"ancestor_record_offset":4184,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3852,"record_offset":8068,"parent_record_offset":8012,"ancestor_record_offset":4184,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3852,"record_offset":10944,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3852,"record_offset":10804,"parent_record_offset":10460,"ancestor_record_offset":4184,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":6024,"parent_record_offset":5984,"ancestor_record_offset":4184,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3852,"record_offset":6148,"parent_record_offset":6104,"ancestor_record_offset":4184,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":6312,"parent_record_offset":6104,"ancestor_record_offset":4184,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3852,"record_offset":6584,"parent_record_offset":6312,"ancestor_record_offset":4184,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":3852,"record_offset":12744,"parent_record_offset":12704,"ancestor_record_offset":4184,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3852,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":4184,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3852,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":4184,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3852,"record_offset":9296,"parent_record_offset":9088,"ancestor_record_offset":4184,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3852,"record_offset":6176,"parent_record_offset":6148,"ancestor_record_offset":4184,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3852,"record_offset":6208,"parent_record_offset":6176,"ancestor_record_offset":4184,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3852,"record_offset":6104,"parent_record_offset":6064,"ancestor_record_offset":4184,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3852,"record_offset":6064,"parent_record_offset":6024,"ancestor_record_offset":4184,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":3852,"record_offset":9088,"parent_record_offset":9024,"ancestor_record_offset":4184,"inlinee":111821,"source_line":106,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::dormer_holes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4CF
// 首条位置: {"module":3852,"inlinee":111823}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":10504,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":10600,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3852,"record_offset":10888,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3852,"record_offset":10916,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3852,"record_offset":10944,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3852,"record_offset":10804,"parent_record_offset":10460,"ancestor_record_offset":10460,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":10460,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10628,"parent_record_offset":10600,"ancestor_record_offset":10460,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10560,"parent_record_offset":10532,"ancestor_record_offset":10460,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3852,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":10460,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":10828,"parent_record_offset":10804,"ancestor_record_offset":10460,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3852,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":10460,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4CD
// 首条位置: {"module":3852,"inlinee":111821}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9208,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3852,"record_offset":9296,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":9088,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":9088,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::dormer_holes::get_cells_from_bbx
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4CC
// 首条位置: {"module":3852,"inlinee":111820}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9624,"parent_record_offset":9024,"ancestor_record_offset":9024,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9924,"parent_record_offset":9024,"ancestor_record_offset":9024,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3852,"record_offset":9652,"parent_record_offset":9024,"ancestor_record_offset":9024,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":10048,"parent_record_offset":9024,"ancestor_record_offset":9024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/dormer_holes.rs
//     evidence={"module":3852,"record_offset":9088,"parent_record_offset":9024,"ancestor_record_offset":9024,"inlinee":111821,"source_line":106,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":9024,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3852,"record_offset":10200,"parent_record_offset":10100,"ancestor_record_offset":9024,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":10132,"parent_record_offset":10100,"ancestor_record_offset":9024,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":9024,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3852,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":9024,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3852,"record_offset":9960,"parent_record_offset":9924,"ancestor_record_offset":9024,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3852,"record_offset":9748,"parent_record_offset":9680,"ancestor_record_offset":9024,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3852,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":9024,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3852,"record_offset":10164,"parent_record_offset":10132,"ancestor_record_offset":9024,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":9024,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3852,"record_offset":9236,"parent_record_offset":9208,"ancestor_record_offset":9024,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3852,"record_offset":9716,"parent_record_offset":9680,"ancestor_record_offset":9024,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3852,"record_offset":10016,"parent_record_offset":9924,"ancestor_record_offset":9024,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":9024,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3852,"record_offset":9208,"parent_record_offset":9088,"ancestor_record_offset":9024,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03852.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3852,"record_offset":9296,"parent_record_offset":9088,"ancestor_record_offset":9024,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/03852.jsonl.gz"}
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
// 完整结构及原始类型记录: dormer_holes.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dormer_holes.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dormer_holes.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::dormer_holes
// 信息 ID: 59554e9633ad48b6070987d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fe5c36d41c30ea6ec29120c4: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17d05cf -> 0x1c4bc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x17e4aed -> 0x1c4bc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::dormer_holes::closure$0
// 信息 ID: f901e88f1fda07c797c4057c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::dormer_holes::dormer_holes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::dormer_holes; module=3852, inline_record=10460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::get_cells_from_bbx
// 信息 ID: e140abba62a34b554ebdfeba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::dormer_holes; module=3852, inline_record=9024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::dormer_holes::world_space_to_grid_coord_unclamped
// 信息 ID: c1d9018dff41fec67cbb9afb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::dormer_holes::dormer_holes; module=3852, inline_record=9088; 不是独立调用边。
// [PDB 内联宿主] system_decorator::dormer_holes::dormer_holes; module=3852, inline_record=9356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
