// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2e026aeaa1f24d80c357278d1af2123e021b9455af7969d41e7035c9df719912
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: maintain_stone_column_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 365, 1226, 1264, 2699
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
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":365,"record_offset":3068,"rva":5124528,"va":5373833648}
// 编译实例 RVAs: 0x4e31b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4221597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e86395bfff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373835112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2183115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e835b0deff","size":5,"soft_fail":false,"target_va":5371651744,"va":5373834854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1411271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e83977eaff","size":5,"soft_fail":false,"target_va":5372425312,"va":5373836578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1085875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e84d6eefff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373835566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1085059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e87d71efff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373834766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t680339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e893610a00","size":5,"soft_fail":false,"target_va":5374516448,"va":5373836104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8052574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e85edf7a00","size":5,"soft_fail":false,"target_va":5381887296,"va":5373834717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cab1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8157961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8097b7c00","size":5,"soft_fail":false,"target_va":5381992912,"va":5373834946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caca80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8163867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e81b927c00","size":5,"soft_fail":false,"target_va":5381999232,"va":5373835360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdcd0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8234630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e886a67d00","size":5,"soft_fail":false,"target_va":5382069456,"va":5373834821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbded0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8233794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e842a37d00","size":5,"soft_fail":false,"target_va":5382069968,"va":5373836169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29141760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e800abbc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373836379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2531223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e86960d9ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5373836274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3690722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8e2503800","size":5,"soft_fail":false,"target_va":5377527120,"va":5373836393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29142254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8eeacbc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373836429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29142513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8f1adbc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373836362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e88ff0ffff","size":5,"soft_fail":false,"target_va":5373832368,"va":5373836316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29145958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e866bbbc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373834165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e860fbffff","size":5,"soft_fail":false,"target_va":5373832560,"va":5373833739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29146835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8d3bebc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373833816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t680816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e870630a00","size":5,"soft_fail":false,"target_va":5374516688,"va":5373835867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2522575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8cf7d2600","size":5,"soft_fail":false,"target_va":5376356912,"va":5373834332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2526458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8fa8c2600","size":5,"soft_fail":false,"target_va":5376360560,"va":5373834097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2522426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e83a7d2600","size":5,"soft_fail":false,"target_va":5376356144,"va":5373833713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8031302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373833648,"raw_hex":"e8468c7a00","size":5,"soft_fail":false,"target_va":5381865424,"va":5373834117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":365,"record_offset":10908,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7636,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":9224,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":6892,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7956,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":365,"record_offset":8044,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":365,"record_offset":6836,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":5980,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send_default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":11460,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7452,"source_line":641,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":6524,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":365,"record_offset":7872,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":11724,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":365,"record_offset":6948,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":365,"record_offset":9252,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":4932,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":3344,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":3920,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":365,"record_offset":8176,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":365,"record_offset":6920,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":9792,"parent_record_offset":9768,"ancestor_record_offset":3068,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":3068,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":10980,"parent_record_offset":10944,"ancestor_record_offset":3068,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":9768,"parent_record_offset":9664,"ancestor_record_offset":3068,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":365,"record_offset":7124,"parent_record_offset":7024,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":365,"record_offset":10944,"parent_record_offset":10908,"ancestor_record_offset":3068,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":7056,"parent_record_offset":7024,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":9664,"parent_record_offset":9632,"ancestor_record_offset":3068,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":9632,"parent_record_offset":9600,"ancestor_record_offset":3068,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":9888,"parent_record_offset":9632,"ancestor_record_offset":3068,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":365,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":365,"record_offset":7440,"parent_record_offset":7404,"ancestor_record_offset":3068,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":365,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":3068,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":11624,"parent_record_offset":11584,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7672,"parent_record_offset":7636,"ancestor_record_offset":3068,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":3068,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7728,"parent_record_offset":7636,"ancestor_record_offset":3068,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":11584,"parent_record_offset":11548,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7756,"parent_record_offset":7636,"ancestor_record_offset":3068,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":9568,"parent_record_offset":9528,"ancestor_record_offset":3068,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":9600,"parent_record_offset":9568,"ancestor_record_offset":3068,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":7896,"parent_record_offset":7872,"ancestor_record_offset":3068,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":365,"record_offset":10228,"parent_record_offset":10196,"ancestor_record_offset":3068,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":365,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":3068,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":365,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":3068,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":365,"record_offset":10336,"parent_record_offset":10228,"ancestor_record_offset":3068,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":365,"record_offset":9932,"parent_record_offset":9528,"ancestor_record_offset":3068,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":365,"record_offset":9492,"parent_record_offset":9456,"ancestor_record_offset":3068,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":365,"record_offset":9528,"parent_record_offset":9492,"ancestor_record_offset":3068,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":365,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":3068,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":365,"record_offset":11484,"parent_record_offset":11460,"ancestor_record_offset":3068,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":3068,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":11548,"parent_record_offset":11484,"ancestor_record_offset":3068,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":6044,"parent_record_offset":6004,"ancestor_record_offset":3068,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":365,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":3068,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":365,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3068,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":365,"record_offset":9704,"parent_record_offset":9664,"ancestor_record_offset":3068,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":365,"record_offset":9732,"parent_record_offset":9704,"ancestor_record_offset":3068,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5332,"parent_record_offset":5304,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5272,"parent_record_offset":5244,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5304,"parent_record_offset":5272,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5476,"parent_record_offset":5304,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":365,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":365,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":365,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":3068,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":365,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":365,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":3068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":365,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":3068,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":365,"record_offset":6576,"parent_record_offset":6548,"ancestor_record_offset":3068,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":365,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":3068,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":365,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":3068,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":365,"record_offset":10128,"parent_record_offset":10028,"ancestor_record_offset":3068,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":365,"record_offset":10092,"parent_record_offset":10064,"ancestor_record_offset":3068,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":365,"record_offset":10064,"parent_record_offset":10028,"ancestor_record_offset":3068,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":365,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":3068,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":365,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":3068,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":365,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":3068,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":365,"record_offset":10196,"parent_record_offset":10128,"ancestor_record_offset":3068,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":365,"record_offset":7984,"parent_record_offset":7956,"ancestor_record_offset":3068,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":365,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":365,"record_offset":6112,"parent_record_offset":6068,"ancestor_record_offset":3068,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":365,"record_offset":7088,"parent_record_offset":7056,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":365,"record_offset":3556,"parent_record_offset":3460,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":365,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":3068,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":5168,"parent_record_offset":5140,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":4516,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":5216,"parent_record_offset":5040,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":365,"record_offset":6656,"parent_record_offset":6576,"ancestor_record_offset":3068,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":365,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":365,"record_offset":5540,"parent_record_offset":5244,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":365,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":365,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":365,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":7472,"parent_record_offset":7440,"ancestor_record_offset":3068,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":8116,"parent_record_offset":8044,"ancestor_record_offset":3068,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":365,"record_offset":4476,"parent_record_offset":4444,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":365,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":3068,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":365,"record_offset":9844,"parent_record_offset":9816,"ancestor_record_offset":3068,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":365,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":365,"record_offset":5940,"parent_record_offset":5004,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":365,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":365,"record_offset":6368,"parent_record_offset":6068,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":365,"record_offset":3520,"parent_record_offset":3492,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":365,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":3068,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":365,"record_offset":6216,"parent_record_offset":6176,"ancestor_record_offset":3068,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":365,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":365,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":365,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":365,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":365,"record_offset":6732,"parent_record_offset":6680,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":365,"record_offset":4220,"parent_record_offset":4112,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":365,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":365,"record_offset":3952,"parent_record_offset":3920,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":365,"record_offset":4084,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":365,"record_offset":4056,"parent_record_offset":4024,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":365,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":365,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":365,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":365,"record_offset":4548,"parent_record_offset":4024,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3656,"parent_record_offset":3624,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3816,"parent_record_offset":3656,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3624,"parent_record_offset":3600,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3432,"parent_record_offset":3400,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":365,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":365,"record_offset":4112,"parent_record_offset":4084,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":365,"record_offset":4360,"parent_record_offset":4112,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":365,"record_offset":4332,"parent_record_offset":4112,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":365,"record_offset":4164,"parent_record_offset":4112,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":365,"record_offset":7500,"parent_record_offset":7440,"ancestor_record_offset":3068,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":365,"record_offset":7532,"parent_record_offset":7404,"ancestor_record_offset":3068,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":365,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":3068,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":365,"record_offset":8872,"parent_record_offset":8848,"ancestor_record_offset":3068,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":365,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":3068,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":365,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":3068,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":365,"record_offset":8848,"parent_record_offset":8708,"ancestor_record_offset":3068,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":365,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":3068,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8760,"parent_record_offset":8736,"ancestor_record_offset":3068,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":3068,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":3068,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":9056,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":9092,"parent_record_offset":8308,"ancestor_record_offset":3068,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":3068,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8284,"parent_record_offset":8224,"ancestor_record_offset":3068,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":3068,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":365,"record_offset":8588,"parent_record_offset":8556,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":365,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":365,"record_offset":8676,"parent_record_offset":8556,"ancestor_record_offset":3068,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":365,"record_offset":8556,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8388,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":365,"record_offset":8708,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":365,"record_offset":8416,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":365,"record_offset":8500,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":365,"record_offset":8964,"parent_record_offset":8336,"ancestor_record_offset":3068,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":4968,"parent_record_offset":4932,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":365,"record_offset":5080,"parent_record_offset":5040,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":5244,"parent_record_offset":5040,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":365,"record_offset":5512,"parent_record_offset":5244,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":365,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":365,"record_offset":5140,"parent_record_offset":5108,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":365,"record_offset":5040,"parent_record_offset":5004,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":365,"record_offset":5004,"parent_record_offset":4968,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
//     evidence={"module":365,"record_offset":10156,"parent_record_offset":10128,"ancestor_record_offset":3068,"inlinee":21445,"source_line":63,"archive":"_pdb/A/modules/00365.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8B5
// 首条位置: {"module":1226,"inlinee":59573}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1226,"record_offset":4616,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1226,"record_offset":4588,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1226,"record_offset":4504,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1226,"record_offset":4560,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":45783,"source_line":75,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1226,"record_offset":4532,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8B4
// 首条位置: {"module":1226,"inlinee":59572}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53C5
// 首条位置: {"module":365,"inlinee":21445}
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
// 完整结构及原始类型记录: maintain_stone_column_colliders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: maintain_stone_column_colliders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: maintain_stone_column_colliders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 信息 ID: f525e584683a83addc0e1ee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e06f537ca23e34f9aa5836: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> @ 0x2d159 -> 0x4e31b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> > @ 0xc631b -> 0x4e31b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 信息 ID: bacf17b2082d5df7e44402e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=4456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 信息 ID: f5c72cf50fec153ba8716d54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1226, inline_record=5244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
// 信息 ID: 8d2126f46a3c00c12af8357c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders; module=365, inline_record=10156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
