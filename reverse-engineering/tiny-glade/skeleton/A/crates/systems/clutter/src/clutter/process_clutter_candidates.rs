// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/process_clutter_candidates.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b0dc34889348353015e26881c49f14281b60aa549bb7f4fa1665836bf99f85fa
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: process_clutter_candidates.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2286, 2292, 3505
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3505,"record_offset":3060,"rva":28095200,"va":5396804320}
// 编译实例 RVAs: 0x1acb2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: process_clutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24055357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8c3f190fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5396805048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: process_clutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24056620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8d4ec90fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396806327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414ec7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: process_clutter_candidates.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6155977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e83711a2ff","size":5,"soft_fail":false,"target_va":5390649280,"va":5396805252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6172769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e861305e00","size":5,"soft_fail":false,"target_va":5402978144,"va":5396805370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17261625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8c79bf8fe","size":5,"soft_fail":false,"target_va":5379544400,"va":5396806020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-19279414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8cad1d9fe","size":5,"soft_fail":false,"target_va":5377527120,"va":5396806529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6172258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8622e5e00","size":5,"soft_fail":false,"target_va":5402978688,"va":5396806425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6172928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e800315e00","size":5,"soft_fail":false,"target_va":5402979344,"va":5396806411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e891f4ffff","size":5,"soft_fail":false,"target_va":5396803040,"va":5396805962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6175317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8553a5e00","size":5,"soft_fail":false,"target_va":5402980128,"va":5396804806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e871fbffff","size":5,"soft_fail":false,"target_va":5396803232,"va":5396804394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t6176180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8b43d5e00","size":5,"soft_fail":false,"target_va":5402980656,"va":5396804471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::color
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12613880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e808873fff","size":5,"soft_fail":false,"target_va":5384192400,"va":5396806275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12613908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8ec863fff","size":5,"soft_fail":false,"target_va":5384192352,"va":5396806255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle3d::show
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12613797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e85b873fff","size":5,"soft_fail":false,"target_va":5384192496,"va":5396806288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20448066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8befcc7fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5396804973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20444182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8ea0bc8fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5396804737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20448235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e815fcc7fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5396804374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h93c6532eedf5b01e (.llvm.12250732642055207085)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17882122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e8f623effe","size":5,"soft_fail":false,"target_va":5378922640,"va":5396804757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18853251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396804320,"raw_hex":"e87d52e0fe","size":5,"soft_fail":false,"target_va":5377952688,"va":5396805934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3505,"record_offset":6924,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":10024,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6980,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3505,"record_offset":5912,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3505,"record_offset":7408,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3505,"record_offset":5972,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3505,"record_offset":10360,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3505,"record_offset":7324,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":9816,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":4864,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":3276,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":3852,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterType::is_olden
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3505,"record_offset":7708,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48756,"source_line":81,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3505,"record_offset":10160,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":106774,"source_line":92,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3505,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":3060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3505,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":3060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3505,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3505,"record_offset":6312,"parent_record_offset":6216,"ancestor_record_offset":3060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3505,"record_offset":10512,"parent_record_offset":10412,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":10444,"parent_record_offset":10412,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":10096,"parent_record_offset":10064,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":3060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":3060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":6432,"parent_record_offset":6184,"ancestor_record_offset":3060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3505,"record_offset":10412,"parent_record_offset":10384,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3505,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":3060,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":10064,"parent_record_offset":10024,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":3060,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6120,"parent_record_offset":6080,"ancestor_record_offset":3060,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6152,"parent_record_offset":6120,"ancestor_record_offset":3060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":7584,"parent_record_offset":7556,"ancestor_record_offset":3060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6008,"parent_record_offset":5972,"ancestor_record_offset":3060,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":6724,"parent_record_offset":6692,"ancestor_record_offset":3060,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3505,"record_offset":6812,"parent_record_offset":6724,"ancestor_record_offset":3060,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3505,"record_offset":6472,"parent_record_offset":6080,"ancestor_record_offset":3060,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":3505,"record_offset":6044,"parent_record_offset":6008,"ancestor_record_offset":3060,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":3505,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":3060,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3505,"record_offset":5940,"parent_record_offset":5912,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3505,"record_offset":6256,"parent_record_offset":6216,"ancestor_record_offset":3060,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3505,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":3060,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5204,"parent_record_offset":5176,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5236,"parent_record_offset":5204,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5408,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3505,"record_offset":5520,"parent_record_offset":5496,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$3::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3505,"record_offset":10196,"parent_record_offset":10160,"ancestor_record_offset":3060,"inlinee":32536,"source_line":263,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3505,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":3060,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3505,"record_offset":6628,"parent_record_offset":6592,"ancestor_record_offset":3060,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3505,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":3060,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3505,"record_offset":6660,"parent_record_offset":6628,"ancestor_record_offset":3060,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3505,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":3060,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3505,"record_offset":6692,"parent_record_offset":6660,"ancestor_record_offset":3060,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3505,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3505,"record_offset":10476,"parent_record_offset":10444,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3505,"record_offset":3488,"parent_record_offset":3392,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3505,"record_offset":7528,"parent_record_offset":7500,"ancestor_record_offset":3060,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3505,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3505,"record_offset":4448,"parent_record_offset":3988,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3505,"record_offset":5148,"parent_record_offset":4972,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3505,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3505,"record_offset":5472,"parent_record_offset":5176,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3505,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3505,"record_offset":6856,"parent_record_offset":6592,"ancestor_record_offset":3060,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3505,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3505,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3505,"record_offset":4408,"parent_record_offset":4376,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3505,"record_offset":6388,"parent_record_offset":6360,"ancestor_record_offset":3060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3505,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3505,"record_offset":5872,"parent_record_offset":4936,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3505,"record_offset":3424,"parent_record_offset":3392,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3505,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":3060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3505,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3505,"record_offset":7648,"parent_record_offset":7616,"ancestor_record_offset":3060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3505,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":9932,"parent_record_offset":9856,"ancestor_record_offset":3060,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":3060,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3505,"record_offset":4152,"parent_record_offset":4044,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3505,"record_offset":3392,"parent_record_offset":3364,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3505,"record_offset":3884,"parent_record_offset":3852,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3505,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3505,"record_offset":3988,"parent_record_offset":3956,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3505,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3505,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3505,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3505,"record_offset":4480,"parent_record_offset":3956,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3588,"parent_record_offset":3556,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3748,"parent_record_offset":3588,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3308,"parent_record_offset":3276,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3364,"parent_record_offset":3332,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3505,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3505,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3505,"record_offset":4292,"parent_record_offset":4044,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3505,"record_offset":4264,"parent_record_offset":4044,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3505,"record_offset":4096,"parent_record_offset":4044,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":4900,"parent_record_offset":4864,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3505,"record_offset":5012,"parent_record_offset":4972,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":5176,"parent_record_offset":4972,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3505,"record_offset":5444,"parent_record_offset":5176,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3505,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3505,"record_offset":5072,"parent_record_offset":5040,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3505,"record_offset":4972,"parent_record_offset":4936,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3505,"record_offset":4936,"parent_record_offset":4900,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/process_clutter_candidates.rs
//     evidence={"module":3505,"record_offset":7556,"parent_record_offset":7528,"ancestor_record_offset":3060,"inlinee":81276,"source_line":74,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/process_clutter_candidates.rs
//     evidence={"module":3505,"record_offset":9992,"parent_record_offset":9816,"ancestor_record_offset":3060,"inlinee":106772,"source_line":90,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D96
// 首条位置: {"module":2292,"inlinee":81302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::reverse
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2292,"record_offset":7172,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":17912,"source_line":540,"archive":"_pdb/A/modules/02292.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$61::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2292,"record_offset":7144,"parent_record_offset":7112,"ancestor_record_offset":7112,"inlinee":12580,"source_line":1565,"archive":"_pdb/A/modules/02292.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D7C
// 首条位置: {"module":2286,"inlinee":81276}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3505,"record_offset":7584,"parent_record_offset":7556,"ancestor_record_offset":7556,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3505,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":7556,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3505,"record_offset":7648,"parent_record_offset":7616,"ancestor_record_offset":7556,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A114
// 首条位置: {"module":3505,"inlinee":106772}
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
// 完整结构及原始类型记录: process_clutter_candidates.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_clutter::clutter::process_clutter_candidates::OrderedCandidate
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>", "alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["system_clutter::clutter::process_clutter_candidates::OrderedCandidate", "alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: process_clutter_candidates.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: process_clutter_candidates.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<array$<u32,2> >,ref$<array$<u32,2> >)
// 候选参数类型: ["ref$<array$<u32,2> >", "ref$<array$<u32,2> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates
// 信息 ID: 9dc0acbf73fa75aada1b60b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6d4c2010f6a13a652e5a76bf: void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x14c2eca -> 0x1acb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0x14e9a9f -> 0x1acb2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$1
// 信息 ID: 8bfc05257ec30bcdf6675d84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_clutter::clutter::process_clutter_candidates::process_clutter_candidates", "captures": "unknown"}
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1 <- core::slice::sort::insertion_sort_shift_left<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; module=2292, inline_record=7112; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; module=2292, inline_record=7240; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; module=2292, inline_record=7372; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; module=2292, inline_record=8408; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1>,alloc::slice::stable_sort::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> > >; module=2292, inline_record=8732; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >; module=2292, inline_record=15488; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,alloc::slice::impl$0::sort_by::closure_env$0<system_clutter::clutter::process_clutter_candidates::OrderedCandidate,system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure_env$1> >; module=2292, inline_record=15968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$2
// 信息 ID: 20bc37567c15fb14746a8b0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_clutter::clutter::process_clutter_candidates::process_clutter_candidates", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates; module=3505, inline_record=7556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::process_clutter_candidates::process_clutter_candidates::closure$3
// 信息 ID: 401b8759395614ddbbb82d26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_clutter::clutter::process_clutter_candidates::process_clutter_candidates", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::process_clutter_candidates::process_clutter_candidates; module=3505, inline_record=9992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
