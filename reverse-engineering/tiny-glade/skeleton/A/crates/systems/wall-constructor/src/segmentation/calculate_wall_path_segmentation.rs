// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0758c2843ed46d7e165e75b29b5b5afc20de3d640576fb66ac44253f73f1cfcf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: calculate_wall_path_segmentation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 374, 1226, 1234, 1263, 1264, 1265, 2717
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":374,"record_offset":6760,"rva":5194304,"va":5373903424}
// 编译实例 RVAs: 0x4f4240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1154531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e81d62eeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373904222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1155551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8215eeeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373905258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3035456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e840512e00","size":5,"soft_fail":false,"target_va":5376939904,"va":5373904443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c971d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8005137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e811267a00","size":5,"soft_fail":false,"target_va":5381910992,"va":5373905850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8006222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e84e2a7a00","size":5,"soft_fail":false,"target_va":5381912416,"va":5373906189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29688364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e82c02c501","size":5,"soft_fail":false,"target_va":5403593360,"va":5373904991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29072434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8329cbb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373905705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2597686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8ca5cd8ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5373904785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3621425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e831423700","size":5,"soft_fail":false,"target_va":5377527120,"va":5373905690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29072964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8449ebb01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373905719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e889efffff","size":5,"soft_fail":false,"target_va":5373901136,"va":5373905346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29076198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8e6aabb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373903925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e877f7ffff","size":5,"soft_fail":false,"target_va":5373901328,"va":5373903508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29077066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e84aaebb01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373903585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8028385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8e1807a00","size":5,"soft_fail":false,"target_va":5381932784,"va":5373904394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1820359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8c7c61b00","size":5,"soft_fail":false,"target_va":5375724768,"va":5373904404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2452816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8506d2500","size":5,"soft_fail":false,"target_va":5376356912,"va":5373904091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2456699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e87b7c2500","size":5,"soft_fail":false,"target_va":5376360560,"va":5373903856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2452657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8b16c2500","size":5,"soft_fail":false,"target_va":5376356144,"va":5373903482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7961543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8c77b7900","size":5,"soft_fail":false,"target_va":5381865424,"va":5373903876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8a2040000","size":5,"soft_fail":false,"target_va":5373906368,"va":5373905177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::new_line
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8123508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e874f47b00","size":5,"soft_fail":false,"target_va":5382028560,"va":5373905047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::new_loop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8123301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373903424,"raw_hex":"e8a5f37b00","size":5,"soft_fail":false,"target_va":5382028368,"va":5373905062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11068,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11632,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11748,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11692,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":374,"record_offset":11860,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":11124,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":9644,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":12088,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":374,"record_offset":11576,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":374,"record_offset":11096,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":374,"record_offset":11156,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":8596,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":7008,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":7584,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":11604,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":12032,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::threshold_range::threshold_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/threshold_range.rs
//     evidence={"module":374,"record_offset":11324,"parent_record_offset":6760,"ancestor_record_offset":6760,"inlinee":21597,"source_line":3,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":6760,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":6760,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":6760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":10072,"parent_record_offset":9964,"ancestor_record_offset":6760,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":12300,"parent_record_offset":12208,"ancestor_record_offset":6760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":6760,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":6760,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":9964,"parent_record_offset":9920,"ancestor_record_offset":6760,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":9920,"parent_record_offset":9876,"ancestor_record_offset":6760,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":10192,"parent_record_offset":9920,"ancestor_record_offset":6760,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":6760,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11452,"parent_record_offset":11412,"ancestor_record_offset":6760,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":10268,"parent_record_offset":10236,"ancestor_record_offset":6760,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11412,"parent_record_offset":11372,"ancestor_record_offset":6760,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":9832,"parent_record_offset":9776,"ancestor_record_offset":6760,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":9876,"parent_record_offset":9832,"ancestor_record_offset":6760,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":9688,"parent_record_offset":9644,"ancestor_record_offset":6760,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":10716,"parent_record_offset":10684,"ancestor_record_offset":6760,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":374,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":6760,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":374,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":6760,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":374,"record_offset":10752,"parent_record_offset":10716,"ancestor_record_offset":6760,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":374,"record_offset":10236,"parent_record_offset":9776,"ancestor_record_offset":6760,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":374,"record_offset":9732,"parent_record_offset":9688,"ancestor_record_offset":6760,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":374,"record_offset":9776,"parent_record_offset":9732,"ancestor_record_offset":6760,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":374,"record_offset":10016,"parent_record_offset":9964,"ancestor_record_offset":6760,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":374,"record_offset":10040,"parent_record_offset":10016,"ancestor_record_offset":6760,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":6760,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":6760,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":9024,"parent_record_offset":8996,"ancestor_record_offset":6760,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":6760,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":8968,"parent_record_offset":8936,"ancestor_record_offset":6760,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":9140,"parent_record_offset":8968,"ancestor_record_offset":6760,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":374,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":6760,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":10552,"parent_record_offset":10500,"ancestor_record_offset":6760,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":374,"record_offset":10332,"parent_record_offset":10300,"ancestor_record_offset":6760,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":374,"record_offset":10648,"parent_record_offset":10396,"ancestor_record_offset":6760,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":374,"record_offset":10364,"parent_record_offset":10332,"ancestor_record_offset":6760,"inlinee":6055,"source_line":99,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":374,"record_offset":10396,"parent_record_offset":10364,"ancestor_record_offset":6760,"inlinee":6056,"source_line":276,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":10580,"parent_record_offset":10500,"ancestor_record_offset":6760,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":10500,"parent_record_offset":10456,"ancestor_record_offset":6760,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":10456,"parent_record_offset":10396,"ancestor_record_offset":6760,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":10300,"parent_record_offset":10268,"ancestor_record_offset":6760,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":10684,"parent_record_offset":10648,"ancestor_record_offset":6760,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":374,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":6760,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":11540,"parent_record_offset":11372,"ancestor_record_offset":6760,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":6760,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":12264,"parent_record_offset":12236,"ancestor_record_offset":6760,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":374,"record_offset":7220,"parent_record_offset":7124,"ancestor_record_offset":6760,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":10888,"parent_record_offset":10852,"ancestor_record_offset":6760,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":6760,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":6760,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":8832,"parent_record_offset":8804,"ancestor_record_offset":6760,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":8180,"parent_record_offset":7720,"ancestor_record_offset":6760,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":8880,"parent_record_offset":8704,"ancestor_record_offset":6760,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":374,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":6760,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":9204,"parent_record_offset":8908,"ancestor_record_offset":6760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":374,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":6760,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":374,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":6760,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":374,"record_offset":11916,"parent_record_offset":11860,"ancestor_record_offset":6760,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":374,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":6760,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":374,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":6760,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":11660,"parent_record_offset":11632,"ancestor_record_offset":6760,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":10148,"parent_record_offset":10120,"ancestor_record_offset":6760,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":6760,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":374,"record_offset":9604,"parent_record_offset":8668,"ancestor_record_offset":6760,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":374,"record_offset":7156,"parent_record_offset":7124,"ancestor_record_offset":6760,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":7184,"parent_record_offset":7156,"ancestor_record_offset":6760,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":374,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":6760,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":374,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":6760,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":374,"record_offset":7884,"parent_record_offset":7776,"ancestor_record_offset":6760,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":374,"record_offset":7124,"parent_record_offset":7096,"ancestor_record_offset":6760,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":374,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":6760,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":374,"record_offset":7748,"parent_record_offset":7720,"ancestor_record_offset":6760,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":374,"record_offset":7720,"parent_record_offset":7688,"ancestor_record_offset":6760,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":374,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":6760,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":374,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":6760,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":374,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":6760,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":374,"record_offset":8212,"parent_record_offset":7688,"ancestor_record_offset":6760,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7320,"parent_record_offset":7288,"ancestor_record_offset":6760,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7480,"parent_record_offset":7320,"ancestor_record_offset":6760,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":6760,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":6760,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":6760,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7096,"parent_record_offset":7064,"ancestor_record_offset":6760,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":374,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":6760,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":374,"record_offset":7776,"parent_record_offset":7748,"ancestor_record_offset":6760,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":374,"record_offset":8024,"parent_record_offset":7776,"ancestor_record_offset":6760,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":374,"record_offset":7996,"parent_record_offset":7776,"ancestor_record_offset":6760,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":374,"record_offset":7828,"parent_record_offset":7776,"ancestor_record_offset":6760,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":10932,"parent_record_offset":10820,"ancestor_record_offset":6760,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":6760,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":8632,"parent_record_offset":8596,"ancestor_record_offset":6760,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":374,"record_offset":8744,"parent_record_offset":8704,"ancestor_record_offset":6760,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":8908,"parent_record_offset":8704,"ancestor_record_offset":6760,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":374,"record_offset":9176,"parent_record_offset":8908,"ancestor_record_offset":6760,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":374,"record_offset":8772,"parent_record_offset":8744,"ancestor_record_offset":6760,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":374,"record_offset":8804,"parent_record_offset":8772,"ancestor_record_offset":6760,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":374,"record_offset":8704,"parent_record_offset":8668,"ancestor_record_offset":6760,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":374,"record_offset":8668,"parent_record_offset":8632,"ancestor_record_offset":6760,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":374,"record_offset":10820,"parent_record_offset":10648,"ancestor_record_offset":6760,"inlinee":21595,"source_line":79,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":374,"record_offset":11372,"parent_record_offset":11324,"ancestor_record_offset":6760,"inlinee":21598,"source_line":109,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::impl$0::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8C4
// 首条位置: {"module":1226,"inlinee":59588}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1226,"record_offset":14248,"parent_record_offset":11764,"ancestor_record_offset":11764,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":15360,"parent_record_offset":11764,"ancestor_record_offset":11764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1226,"record_offset":11860,"parent_record_offset":11764,"ancestor_record_offset":11764,"inlinee":59590,"source_line":52,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11764,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":11764,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":14028,"parent_record_offset":14004,"ancestor_record_offset":11764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":12304,"parent_record_offset":12204,"ancestor_record_offset":11764,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1226,"record_offset":14004,"parent_record_offset":13912,"ancestor_record_offset":11764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":13940,"parent_record_offset":13912,"ancestor_record_offset":11764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12204,"parent_record_offset":12172,"ancestor_record_offset":11764,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12172,"parent_record_offset":12140,"ancestor_record_offset":11764,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":12424,"parent_record_offset":12172,"ancestor_record_offset":11764,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1226,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":11764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12600,"parent_record_offset":12560,"ancestor_record_offset":11764,"inlinee":7706,"source_line":124,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12044,"parent_record_offset":12008,"ancestor_record_offset":11764,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":11764,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1226,"record_offset":11972,"parent_record_offset":11936,"ancestor_record_offset":11764,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1226,"record_offset":12140,"parent_record_offset":12044,"ancestor_record_offset":11764,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1226,"record_offset":11936,"parent_record_offset":11860,"ancestor_record_offset":11764,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1226,"record_offset":12240,"parent_record_offset":12204,"ancestor_record_offset":11764,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1226,"record_offset":12268,"parent_record_offset":12240,"ancestor_record_offset":11764,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1226,"record_offset":12108,"parent_record_offset":12084,"ancestor_record_offset":11764,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1226,"record_offset":13040,"parent_record_offset":13008,"ancestor_record_offset":11764,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1226,"record_offset":13008,"parent_record_offset":12600,"ancestor_record_offset":11764,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1226,"record_offset":13232,"parent_record_offset":12600,"ancestor_record_offset":11764,"inlinee":6259,"source_line":493,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1226,"record_offset":13968,"parent_record_offset":13940,"ancestor_record_offset":11764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1226,"record_offset":12084,"parent_record_offset":12044,"ancestor_record_offset":11764,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":13864,"parent_record_offset":11860,"ancestor_record_offset":11764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1226,"record_offset":13156,"parent_record_offset":13132,"ancestor_record_offset":11764,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1226,"record_offset":13104,"parent_record_offset":13068,"ancestor_record_offset":11764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1226,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":11764,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1226,"record_offset":13132,"parent_record_offset":13068,"ancestor_record_offset":11764,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1226,"record_offset":13068,"parent_record_offset":13040,"ancestor_record_offset":11764,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1226,"record_offset":13200,"parent_record_offset":12600,"ancestor_record_offset":11764,"inlinee":59592,"source_line":43,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":374,"record_offset":3068,"rva":5193296,"va":5373902416}
// 编译实例 RVAs: 0x4f3e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4289244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e8248dbeff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373902759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1153416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e87866eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373903123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7985266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e872d87900","size":5,"soft_fail":false,"target_va":5381888304,"va":5373903033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8009001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e829357a00","size":5,"soft_fail":false,"target_va":5381912416,"va":5373903410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29075541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e855a8bb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373903334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::init_this_update
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1071494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e886591000","size":5,"soft_fail":false,"target_va":5374974464,"va":5373902965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1071878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e8065b1000","size":5,"soft_fail":false,"target_va":5374974864,"va":5373902981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373902416,"raw_hex":"e8ec010000","size":5,"soft_fail":false,"target_va":5373903424,"va":5373902927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":374,"record_offset":4140,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":374,"record_offset":3288,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":374,"record_offset":3828,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":6728,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":5304,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":374,"record_offset":4196,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":374,"record_offset":5276,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":5192,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":5552,"parent_record_offset":5488,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":5520,"parent_record_offset":5488,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":5488,"parent_record_offset":5456,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3068,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":374,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3068,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":374,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":3068,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":374,"record_offset":3352,"parent_record_offset":3312,"ancestor_record_offset":3068,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":374,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3068,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":374,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":3068,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":374,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":3068,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":374,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":374,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":3068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":374,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3068,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":374,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":3068,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":374,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3068,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":3420,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":5736,"parent_record_offset":5704,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":374,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3068,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":3960,"parent_record_offset":3880,"ancestor_record_offset":3068,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":374,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":374,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":374,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":374,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":374,"record_offset":3484,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":374,"record_offset":3672,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3068,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":3520,"parent_record_offset":3484,"ancestor_record_offset":3068,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":374,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":374,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":374,"record_offset":4036,"parent_record_offset":3984,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":374,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3068,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":374,"record_offset":4952,"parent_record_offset":4928,"ancestor_record_offset":3068,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":374,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3068,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":374,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3068,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":374,"record_offset":4928,"parent_record_offset":4708,"ancestor_record_offset":3068,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":374,"record_offset":4272,"parent_record_offset":4244,"ancestor_record_offset":3068,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":3068,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4816,"parent_record_offset":4708,"ancestor_record_offset":3068,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":3068,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":5136,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":3068,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3068,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4300,"parent_record_offset":4244,"ancestor_record_offset":3068,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":3068,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":374,"record_offset":4588,"parent_record_offset":4560,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":374,"record_offset":4616,"parent_record_offset":4588,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":374,"record_offset":4676,"parent_record_offset":4560,"ancestor_record_offset":3068,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":374,"record_offset":4560,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4392,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":374,"record_offset":4708,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":374,"record_offset":4420,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":374,"record_offset":4504,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":374,"record_offset":5044,"parent_record_offset":4348,"ancestor_record_offset":3068,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x545B
// 首条位置: {"module":374,"inlinee":21595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":10932,"parent_record_offset":10820,"ancestor_record_offset":10820,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":10852,"parent_record_offset":10820,"ancestor_record_offset":10820,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":10888,"parent_record_offset":10852,"ancestor_record_offset":10820,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":10820,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x545E
// 首条位置: {"module":374,"inlinee":21598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11412,"parent_record_offset":11372,"ancestor_record_offset":11372,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":11540,"parent_record_offset":11372,"ancestor_record_offset":11372,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":11372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11452,"parent_record_offset":11412,"ancestor_record_offset":11372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":11508,"parent_record_offset":11412,"ancestor_record_offset":11372,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":374,"record_offset":13312,"rva":5197248,"va":5373906368}
// 编译实例 RVAs: 0x4f4dc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1156973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e89358eeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373906664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1158904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e80851eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373908611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1156756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e86c59eeff","size":5,"soft_fail":false,"target_va":5372749744,"va":5373906495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca94a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8077788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8dc417b00","size":5,"soft_fail":false,"target_va":5381985440,"va":5373907647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb7610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8135884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8cc247c00","size":5,"soft_fail":false,"target_va":5382043152,"va":5373907263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8350760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8286c7f00","size":5,"soft_fail":false,"target_va":5382259200,"va":5373908435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cecb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8353100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e84c757f00","size":5,"soft_fail":false,"target_va":5382261552,"va":5373908447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29686691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8a3fbc401","size":5,"soft_fail":false,"target_va":5403593392,"va":5373906696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29071626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e80a99bb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373906513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3617691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e89b333700","size":5,"soft_fail":false,"target_va":5377527120,"va":5373909424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29069230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8ae8fbb01","size":5,"soft_fail":false,"target_va":5402978176,"va":5373908941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29069801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8e991bb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373909074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29145433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e859b9bc01","size":5,"soft_fail":false,"target_va":5403054688,"va":5373909250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29070451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e87394bb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373908696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29070594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e80295bb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373908745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29069882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e83a92bb01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373909057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e874070000","size":5,"soft_fail":false,"target_va":5373909952,"va":5373908039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t8121831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373906368,"raw_hex":"e8e7ed7b00","size":5,"soft_fail":false,"target_va":5382028304,"va":5373906468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16308,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16340,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":9718,"source_line":1487,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":13564,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16864,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":374,"record_offset":14644,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":16780,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":15336,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":17364,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":374,"record_offset":16536,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21601,"source_line":362,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":374,"record_offset":14868,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21600,"source_line":338,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":13844,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21409,"source_line":1081,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_coord_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":17056,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21602,"source_line":161,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::measure_length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":17156,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21605,"source_line":411,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":13536,"parent_record_offset":13312,"ancestor_record_offset":13312,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":13312,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":13312,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":13312,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":13312,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":14236,"parent_record_offset":14132,"ancestor_record_offset":13312,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":13680,"parent_record_offset":13648,"ancestor_record_offset":13312,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":17516,"parent_record_offset":17416,"ancestor_record_offset":13312,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":17448,"parent_record_offset":17416,"ancestor_record_offset":13312,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":16440,"parent_record_offset":16408,"ancestor_record_offset":13312,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":13648,"parent_record_offset":13620,"ancestor_record_offset":13312,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":14092,"parent_record_offset":14052,"ancestor_record_offset":13312,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":13312,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":13768,"parent_record_offset":13620,"ancestor_record_offset":13312,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":17416,"parent_record_offset":17388,"ancestor_record_offset":13312,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16408,"parent_record_offset":16340,"ancestor_record_offset":13312,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":13312,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":14396,"parent_record_offset":14008,"ancestor_record_offset":13312,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16380,"parent_record_offset":16340,"ancestor_record_offset":13312,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16504,"parent_record_offset":16340,"ancestor_record_offset":13312,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":15504,"parent_record_offset":15432,"ancestor_record_offset":13312,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":14052,"parent_record_offset":14008,"ancestor_record_offset":13312,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":15368,"parent_record_offset":15336,"ancestor_record_offset":13312,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":16172,"parent_record_offset":16140,"ancestor_record_offset":13312,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":374,"record_offset":16240,"parent_record_offset":16172,"ancestor_record_offset":13312,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":374,"record_offset":15844,"parent_record_offset":15432,"ancestor_record_offset":13312,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":374,"record_offset":14008,"parent_record_offset":13968,"ancestor_record_offset":13312,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":374,"record_offset":13592,"parent_record_offset":13564,"ancestor_record_offset":13312,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":374,"record_offset":15400,"parent_record_offset":15368,"ancestor_record_offset":13312,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":374,"record_offset":15432,"parent_record_offset":15400,"ancestor_record_offset":13312,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":374,"record_offset":14180,"parent_record_offset":14132,"ancestor_record_offset":13312,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":374,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13312,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":374,"record_offset":14748,"parent_record_offset":14672,"ancestor_record_offset":13312,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":14484,"parent_record_offset":14456,"ancestor_record_offset":13312,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":15248,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":15276,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":374,"record_offset":13908,"parent_record_offset":13844,"ancestor_record_offset":13312,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":374,"record_offset":16472,"parent_record_offset":16340,"ancestor_record_offset":13312,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":374,"record_offset":17244,"parent_record_offset":17220,"ancestor_record_offset":13312,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":16036,"parent_record_offset":15996,"ancestor_record_offset":13312,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":13312,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":14428,"parent_record_offset":14396,"ancestor_record_offset":13312,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":15476,"parent_record_offset":15432,"ancestor_record_offset":13312,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":374,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":13312,"inlinee":15614,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":13312,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":15892,"parent_record_offset":15868,"ancestor_record_offset":13312,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":16140,"parent_record_offset":15916,"ancestor_record_offset":13312,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":17220,"parent_record_offset":17156,"ancestor_record_offset":13312,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":17304,"parent_record_offset":17156,"ancestor_record_offset":13312,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":16652,"parent_record_offset":16608,"ancestor_record_offset":13312,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":13312,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":16064,"parent_record_offset":16036,"ancestor_record_offset":13312,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":17480,"parent_record_offset":17448,"ancestor_record_offset":13312,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":374,"record_offset":14968,"parent_record_offset":14928,"ancestor_record_offset":13312,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":13312,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":374,"record_offset":14720,"parent_record_offset":14672,"ancestor_record_offset":13312,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":374,"record_offset":14672,"parent_record_offset":14644,"ancestor_record_offset":13312,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":13880,"parent_record_offset":13844,"ancestor_record_offset":13312,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":374,"record_offset":14820,"parent_record_offset":14796,"ancestor_record_offset":13312,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":17388,"parent_record_offset":17364,"ancestor_record_offset":13312,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":14312,"parent_record_offset":14284,"ancestor_record_offset":13312,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":14520,"parent_record_offset":14396,"ancestor_record_offset":13312,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":374,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":13312,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":13312,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":14796,"parent_record_offset":14772,"ancestor_record_offset":13312,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":374,"record_offset":16920,"parent_record_offset":16892,"ancestor_record_offset":13312,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":13312,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15156,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15100,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":374,"record_offset":17276,"parent_record_offset":17156,"ancestor_record_offset":13312,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":374,"record_offset":15212,"parent_record_offset":15180,"ancestor_record_offset":13312,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":16608,"parent_record_offset":16536,"ancestor_record_offset":13312,"inlinee":21409,"source_line":1081,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::iter_coords
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":17184,"parent_record_offset":17156,"ancestor_record_offset":13312,"inlinee":21606,"source_line":342,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$11::sub
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":17096,"parent_record_offset":17056,"ancestor_record_offset":13312,"inlinee":21604,"source_line":1134,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":15304,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":14928,"parent_record_offset":14868,"ancestor_record_offset":13312,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8BD
// 首条位置: {"module":1226,"inlinee":59581}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1226,"record_offset":7228,"parent_record_offset":7176,"ancestor_record_offset":7176,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5460
// 首条位置: {"module":374,"inlinee":21600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":15248,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":15276,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15156,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15100,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":15304,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":14928,"parent_record_offset":14868,"ancestor_record_offset":14868,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":374,"record_offset":14968,"parent_record_offset":14928,"ancestor_record_offset":14868,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":15124,"parent_record_offset":15100,"ancestor_record_offset":14868,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":14868,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":374,"record_offset":15212,"parent_record_offset":15180,"ancestor_record_offset":14868,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5461
// 首条位置: {"module":374,"inlinee":21601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":374,"record_offset":16608,"parent_record_offset":16536,"ancestor_record_offset":16536,"inlinee":21409,"source_line":1081,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":16580,"parent_record_offset":16536,"ancestor_record_offset":16536,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":374,"record_offset":16744,"parent_record_offset":16608,"ancestor_record_offset":16536,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":16652,"parent_record_offset":16608,"ancestor_record_offset":16536,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":16536,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":374,"record_offset":16708,"parent_record_offset":16608,"ancestor_record_offset":16536,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 373
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":374,"record_offset":18192,"rva":5200832,"va":5373909952}
// 编译实例 RVAs: 0x4f5bc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1160856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e86849eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373910563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1970559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e87f111e00","size":5,"soft_fail":false,"target_va":5375881440,"va":5373910876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cad4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8091784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e888787b00","size":5,"soft_fail":false,"target_va":5382001856,"va":5373910067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8120950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e876ea7b00","size":5,"soft_fail":false,"target_va":5382031392,"va":5373910437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8122284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e8acef7b00","size":5,"soft_fail":false,"target_va":5382032368,"va":5373910079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb7610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8132676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e844187c00","size":5,"soft_fail":false,"target_va":5382043152,"va":5373910471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29683935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e8dff0c401","size":5,"soft_fail":false,"target_va":5403594096,"va":5373910156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29068426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e88a8cbb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373910721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29068671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e87f8dbb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373910668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29068138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e86a8bbb01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373910801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8117848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373909952,"raw_hex":"e858de7b00","size":5,"soft_fail":false,"target_va":5382028304,"va":5373910451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":18508,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":374,"record_offset":18396,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":18912,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":374,"record_offset":18480,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":374,"record_offset":18672,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":18884,"parent_record_offset":18192,"ancestor_record_offset":18192,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":18192,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":374,"record_offset":19088,"parent_record_offset":18988,"ancestor_record_offset":18192,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":19020,"parent_record_offset":18988,"ancestor_record_offset":18192,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":18988,"parent_record_offset":18960,"ancestor_record_offset":18192,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":18420,"parent_record_offset":18396,"ancestor_record_offset":18192,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":18760,"parent_record_offset":18736,"ancestor_record_offset":18192,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":18784,"parent_record_offset":18760,"ancestor_record_offset":18192,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":374,"record_offset":18572,"parent_record_offset":18536,"ancestor_record_offset":18192,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":18824,"parent_record_offset":18672,"ancestor_record_offset":18192,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":374,"record_offset":18852,"parent_record_offset":18672,"ancestor_record_offset":18192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":18604,"parent_record_offset":18536,"ancestor_record_offset":18192,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":374,"record_offset":18536,"parent_record_offset":18508,"ancestor_record_offset":18192,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":374,"record_offset":18444,"parent_record_offset":18420,"ancestor_record_offset":18192,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":18192,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":19052,"parent_record_offset":19020,"ancestor_record_offset":18192,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":374,"record_offset":18736,"parent_record_offset":18712,"ancestor_record_offset":18192,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":374,"record_offset":18712,"parent_record_offset":18672,"ancestor_record_offset":18192,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":18936,"parent_record_offset":18912,"ancestor_record_offset":18192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate
// 记录形式: compiled_procedure; instances=1
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":374,"record_offset":19384,"rva":5201808,"va":5373910928}
// 编译实例 RVAs: 0x4f5f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc6d50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8195278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e8ce0c7d00","size":5,"soft_fail":false,"target_va":5382106448,"va":5373911165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7630
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8197533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e89d157d00","size":5,"soft_fail":false,"target_va":5382108720,"va":5373911182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc87b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8201977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e8f9267d00","size":5,"soft_fail":false,"target_va":5382113200,"va":5373911218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29067522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e80289bb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373911625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29067738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e8da89bb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373911601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8117303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373910928,"raw_hex":"e837dc7b00","size":5,"soft_fail":false,"target_va":5382028304,"va":5373910996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":20712,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":20656,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":19864,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":19620,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::start
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":19836,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21610,"source_line":330,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":19592,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":19892,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21487,"source_line":225,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":19980,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":20416,"parent_record_offset":19980,"ancestor_record_offset":19384,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":20384,"parent_record_offset":20008,"ancestor_record_offset":19384,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":374,"record_offset":19932,"parent_record_offset":19892,"ancestor_record_offset":19384,"inlinee":21488,"source_line":2775,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20528,"parent_record_offset":20496,"ancestor_record_offset":19384,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20584,"parent_record_offset":20496,"ancestor_record_offset":19384,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":19384,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":374,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":19384,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20280,"parent_record_offset":20248,"ancestor_record_offset":19384,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":19384,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20060,"parent_record_offset":20008,"ancestor_record_offset":19384,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":19384,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":374,"record_offset":20768,"parent_record_offset":20736,"ancestor_record_offset":19384,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":374,"record_offset":19664,"parent_record_offset":19620,"ancestor_record_offset":19384,"inlinee":21609,"source_line":48,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20248,"parent_record_offset":20008,"ancestor_record_offset":19384,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20448,"parent_record_offset":19980,"ancestor_record_offset":19384,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":374,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19384,"inlinee":21481,"source_line":198,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":19384,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 439
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAAF
// 首条位置: {"module":1263,"inlinee":60079}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E24
// 首条位置: {"module":2717,"inlinee":89636}
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
// 完整结构及原始类型记录: calculate_wall_path_segmentation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$10::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::impl$10::clone", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: calculate_wall_path_segmentation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: calculate_wall_path_segmentation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
// 信息 ID: 83f6d373438175fdae87dbc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=374, inline_record=16536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
// 信息 ID: b54efc90d9035b0eb97c2afa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=374, inline_record=14868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
// 信息 ID: 3a0e8b37998ef4bc660f5b65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x4f5447 -> 0x4f5bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x4f54a9 -> 0x4f5bc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
// 信息 ID: 3ccd7ce72c729a2b13993404
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x4f404f -> 0x4f4240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
// 信息 ID: 55a0c56977026cc2245e24c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=10820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
// 信息 ID: 95671025981478e5d78b8fe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=11372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation
// 信息 ID: 6d10fcc3b4935e73fbb48ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 864568abef8729d36bd8a4d5: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xaf454 -> 0x4f3e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x2d4492 -> 0x4f3e50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::impl$0::clone
// 信息 ID: c9d22d36fee65cf20a9f6de9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$10::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1226, inline_record=11764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
// 信息 ID: f369d8db0e54a370cbaba509
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x4f4919 -> 0x4f4dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8> @ 0xcbbe67 -> 0x4f4dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8> @ 0xcbbf4e -> 0x4f4dc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure$0
// 信息 ID: 998099af9b6428a41bf6bccf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0 <- alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; module=1226, inline_record=7176; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; module=1226, inline_record=7340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate
// 信息 ID: 46647bd73654d8cdae473130
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0> @ 0xca9576 -> 0x4f5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0> @ 0xca9627 -> 0x4f5f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
// 信息 ID: 55bfac8439eaf67f88bfbe36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$6::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=2036; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::tuple_windows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1263, inline_record=12268; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1265, inline_record=6104; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1265, inline_record=8004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
// 信息 ID: 7c2ff8c55de81f160673d6fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> > >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; module=2717, inline_record=1744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
