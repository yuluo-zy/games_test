// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/world_raster.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 823aa061c74056d9da861bfa43f62bedb8ea8a9730e88408b979a0f3accc1e9b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: world_raster.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 114, 136, 364, 392, 406, 409, 412, 413, 421, 424, 430, 431, 448, 452, 459, 478, 498, 499, 1011, 1012, 1325, 1339, 1365, 1368, 1398, 1423, 1438, 1440, 2091, 2154, 2374, 2395, 2414, 2830, 3369, 3381, 3386, 3513, 3514, 3515, 3607, 3608, 3609, 3613, 3621, 4205, 4206, 4604
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use ahash; // namespace candidate
//   package identities: [{"name":"ahash","source":"PDB_registry_path_only","version":"0.8.11"}]
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: []
// use zeno; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::new<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":4420,"rva":16978720,"va":5385687840}
// 编译实例 RVAs: 0x1031320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12938283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e8d5933aff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385687974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12938697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e837923aff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385688404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12938139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e865943aff","size":5,"soft_fail":false,"target_va":5372749744,"va":5385687878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406efeb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9705182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e822e96bff","size":5,"soft_fail":false,"target_va":5375983280,"va":5385688457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17905213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e83d361101","size":5,"soft_fail":false,"target_va":5403593392,"va":5385688174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t17289853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385687840,"raw_hex":"e87dd20701","size":5,"soft_fail":false,"target_va":5402978144,"va":5385688286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":4572,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":5872,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] tabloid::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":1398,"record_offset":5240,"parent_record_offset":4420,"ancestor_record_offset":4420,"inlinee":27941,"source_line":46,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":4420,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4420,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":4420,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":4420,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":4420,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":4680,"parent_record_offset":4656,"ancestor_record_offset":4420,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":4420,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":4420,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":4420,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":4420,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":4768,"parent_record_offset":4628,"ancestor_record_offset":4420,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":4420,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":5568,"parent_record_offset":5296,"ancestor_record_offset":4420,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":5332,"parent_record_offset":5296,"ancestor_record_offset":4420,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1398,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":4420,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1398,"record_offset":4600,"parent_record_offset":4572,"ancestor_record_offset":4420,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":4420,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":4420,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":4420,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::clear_buffer<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":6296,"rva":16979440,"va":5385688560}
// 编译实例 RVAs: 0x10315f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2380874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8b6abdbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385689013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17904112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8f0311101","size":5,"soft_fail":false,"target_va":5403593392,"va":5385689275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17289371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e89bd00701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385689312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17291061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e835d70701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385689062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8bef8ffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385688621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17291960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8b8da0701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385688691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9332319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8a19971ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385689226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9328438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e8caa871ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385688993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9332453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385688560,"raw_hex":"e81b9971ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385688592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-2729","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385688560,"raw_hex":"e957f5ffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385689300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":7040,"parent_record_offset":6296,"ancestor_record_offset":6296,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":8200,"parent_record_offset":6296,"ancestor_record_offset":6296,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":6456,"parent_record_offset":6296,"ancestor_record_offset":6296,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":7188,"parent_record_offset":6296,"ancestor_record_offset":6296,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":6296,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":6296,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":6296,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8524,"parent_record_offset":8496,"ancestor_record_offset":6296,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8556,"parent_record_offset":8524,"ancestor_record_offset":6296,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8728,"parent_record_offset":8556,"ancestor_record_offset":6296,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":6296,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":6296,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":6676,"parent_record_offset":6580,"ancestor_record_offset":6296,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":8420,"parent_record_offset":8392,"ancestor_record_offset":6296,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":7784,"parent_record_offset":7324,"ancestor_record_offset":6296,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":8468,"parent_record_offset":8296,"ancestor_record_offset":6296,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":6988,"parent_record_offset":6960,"ancestor_record_offset":6296,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":8792,"parent_record_offset":8496,"ancestor_record_offset":6296,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":6296,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":6296,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":7144,"parent_record_offset":7100,"ancestor_record_offset":6296,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":6296,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":9192,"parent_record_offset":8264,"ancestor_record_offset":6296,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":6296,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":6296,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":6296,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":6296,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":7488,"parent_record_offset":7380,"ancestor_record_offset":6296,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":7100,"parent_record_offset":7068,"ancestor_record_offset":6296,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1398,"record_offset":7068,"parent_record_offset":7040,"ancestor_record_offset":6296,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":6580,"parent_record_offset":6552,"ancestor_record_offset":6296,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":7220,"parent_record_offset":7188,"ancestor_record_offset":6296,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":7352,"parent_record_offset":7324,"ancestor_record_offset":6296,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":7324,"parent_record_offset":7292,"ancestor_record_offset":6296,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":6296,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":6296,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6296,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":7816,"parent_record_offset":7292,"ancestor_record_offset":6296,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":6296,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6936,"parent_record_offset":6776,"ancestor_record_offset":6296,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6296,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6492,"parent_record_offset":6456,"ancestor_record_offset":6296,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":6296,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":6296,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":6520,"parent_record_offset":6492,"ancestor_record_offset":6296,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":7380,"parent_record_offset":7352,"ancestor_record_offset":6296,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":7628,"parent_record_offset":7380,"ancestor_record_offset":6296,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":7600,"parent_record_offset":7380,"ancestor_record_offset":6296,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":7432,"parent_record_offset":7380,"ancestor_record_offset":6296,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":8232,"parent_record_offset":8200,"ancestor_record_offset":6296,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":8332,"parent_record_offset":8296,"ancestor_record_offset":6296,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":8496,"parent_record_offset":8296,"ancestor_record_offset":6296,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":8764,"parent_record_offset":8496,"ancestor_record_offset":6296,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":8360,"parent_record_offset":8332,"ancestor_record_offset":6296,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":8392,"parent_record_offset":8360,"ancestor_record_offset":6296,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":8296,"parent_record_offset":8264,"ancestor_record_offset":6296,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":8264,"parent_record_offset":8232,"ancestor_record_offset":6296,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":18276,"rva":16982912,"va":5385692032}
// 编译实例 RVAs: 0x1032380, 0x1db8950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12943207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e899803aff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385692898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12943802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8467e3aff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385693509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2384458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8b69ddbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385692597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14111a510
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t949249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8017c0e00","size":5,"soft_fail":false,"target_va":5386642704,"va":5385693450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14111ad50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t951337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e829840e00","size":5,"soft_fail":false,"target_va":5386644816,"va":5385693474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14111b590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t953502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e89e8c0e00","size":5,"soft_fail":false,"target_va":5386646928,"va":5385693421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17284525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8adbd0701","size":5,"soft_fail":false,"target_va":5402978144,"va":5385693614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3827229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8e399c5ff","size":5,"soft_fail":false,"target_va":5381865936,"va":5385693160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8166484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8ac6383ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5385693599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17285015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e897bf0701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385693668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17285247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e87fc00701","size":5,"soft_fail":false,"target_va":5402978880,"va":5385693628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17285493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e875c10701","size":5,"soft_fail":false,"target_va":5402979152,"va":5385693654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8d3e4ffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385693528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17287477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e835c90701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385692646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-5437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8c3eaffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385692200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t17288378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8bacc0701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385692273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9335911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8998b71ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385692818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-9332022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8ca9a71ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385692577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9336031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385692032,"raw_hex":"e8218b71ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385692170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3ab327903ee31098 (.llvm.15844595411771089199)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1046470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399873872,"raw_hex":"e8c6f70f00","size":5,"soft_fail":false,"target_va":5400920912,"va":5399874437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zeno::mask::Mask$LT$D$GT$::blend_into
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1048078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399873872,"raw_hex":"e80efe0f00","size":5,"soft_fail":false,"target_va":5400923344,"va":5399875261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":22032,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":21392,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1398,"record_offset":22684,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1398,"record_offset":21728,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1398,"record_offset":18644,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":22936,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":22716,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::ws_to_px
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1398,"record_offset":21820,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":64952,"source_line":141,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":20344,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":18756,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":19332,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::format
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-d097fe939fcc1ba4/b8e8646/src/mask.rs
//     evidence={"module":1398,"record_offset":22596,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":64959,"source_line":120,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-d097fe939fcc1ba4/b8e8646/src/mask.rs
//     evidence={"module":1398,"record_offset":22568,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":64958,"source_line":69,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::size
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-d097fe939fcc1ba4/b8e8646/src/mask.rs
//     evidence={"module":1398,"record_offset":22652,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":64961,"source_line":140,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] zeno::mask::Mask::style
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/zeno-d097fe939fcc1ba4/b8e8646/src/mask.rs
//     evidence={"module":1398,"record_offset":22624,"parent_record_offset":18276,"ancestor_record_offset":18276,"inlinee":64960,"source_line":107,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":21588,"parent_record_offset":21564,"ancestor_record_offset":18276,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":21612,"parent_record_offset":21588,"ancestor_record_offset":18276,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":18276,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":21564,"parent_record_offset":21476,"ancestor_record_offset":18276,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":22868,"parent_record_offset":22768,"ancestor_record_offset":18276,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":22800,"parent_record_offset":22768,"ancestor_record_offset":18276,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":22168,"parent_record_offset":22136,"ancestor_record_offset":18276,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":21476,"parent_record_offset":21448,"ancestor_record_offset":18276,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":21448,"parent_record_offset":21420,"ancestor_record_offset":18276,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":21684,"parent_record_offset":21448,"ancestor_record_offset":18276,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":22768,"parent_record_offset":22740,"ancestor_record_offset":18276,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":22136,"parent_record_offset":22096,"ancestor_record_offset":18276,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":18276,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1398,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":18276,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1398,"record_offset":21532,"parent_record_offset":21508,"ancestor_record_offset":18276,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20744,"parent_record_offset":20716,"ancestor_record_offset":18276,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20796,"parent_record_offset":20772,"ancestor_record_offset":18276,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20772,"parent_record_offset":20744,"ancestor_record_offset":18276,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20684,"parent_record_offset":20656,"ancestor_record_offset":18276,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20716,"parent_record_offset":20684,"ancestor_record_offset":18276,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":20888,"parent_record_offset":20716,"ancestor_record_offset":18276,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":21000,"parent_record_offset":20976,"ancestor_record_offset":18276,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":20820,"parent_record_offset":20796,"ancestor_record_offset":18276,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1398,"record_offset":22832,"parent_record_offset":22800,"ancestor_record_offset":18276,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":18968,"parent_record_offset":18872,"ancestor_record_offset":18276,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":18276,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":18276,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":18276,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":20580,"parent_record_offset":20552,"ancestor_record_offset":18276,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":19928,"parent_record_offset":19468,"ancestor_record_offset":18276,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":20628,"parent_record_offset":20452,"ancestor_record_offset":18276,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":19280,"parent_record_offset":19252,"ancestor_record_offset":18276,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":18716,"parent_record_offset":18692,"ancestor_record_offset":18276,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":20952,"parent_record_offset":20656,"ancestor_record_offset":18276,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":19704,"parent_record_offset":19680,"ancestor_record_offset":18276,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":19680,"parent_record_offset":19656,"ancestor_record_offset":18276,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":18276,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":19888,"parent_record_offset":19856,"ancestor_record_offset":18276,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":18276,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":20844,"parent_record_offset":20820,"ancestor_record_offset":18276,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":21352,"parent_record_offset":20416,"ancestor_record_offset":18276,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":18904,"parent_record_offset":18872,"ancestor_record_offset":18276,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":18932,"parent_record_offset":18904,"ancestor_record_offset":18276,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":19656,"parent_record_offset":19632,"ancestor_record_offset":18276,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":18276,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":18276,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":21752,"parent_record_offset":21728,"ancestor_record_offset":18276,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":19632,"parent_record_offset":19524,"ancestor_record_offset":18276,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":18872,"parent_record_offset":18844,"ancestor_record_offset":18276,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":19364,"parent_record_offset":19332,"ancestor_record_offset":18276,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":19496,"parent_record_offset":19468,"ancestor_record_offset":18276,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":19468,"parent_record_offset":19436,"ancestor_record_offset":18276,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":19412,"parent_record_offset":19388,"ancestor_record_offset":18276,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":19436,"parent_record_offset":19412,"ancestor_record_offset":18276,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":18276,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":19960,"parent_record_offset":19436,"ancestor_record_offset":18276,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":19068,"parent_record_offset":19036,"ancestor_record_offset":18276,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":19228,"parent_record_offset":19068,"ancestor_record_offset":18276,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":19036,"parent_record_offset":19012,"ancestor_record_offset":18276,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":18788,"parent_record_offset":18756,"ancestor_record_offset":18276,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":19252,"parent_record_offset":19228,"ancestor_record_offset":18276,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":18844,"parent_record_offset":18812,"ancestor_record_offset":18276,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":18812,"parent_record_offset":18788,"ancestor_record_offset":18276,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":19524,"parent_record_offset":19496,"ancestor_record_offset":18276,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":19772,"parent_record_offset":19524,"ancestor_record_offset":18276,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":19744,"parent_record_offset":19524,"ancestor_record_offset":18276,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":19576,"parent_record_offset":19524,"ancestor_record_offset":18276,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$15::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21916,"parent_record_offset":21820,"ancestor_record_offset":18276,"inlinee":64956,"source_line":1031,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$3::div_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21852,"parent_record_offset":21820,"ancestor_record_offset":18276,"inlinee":64954,"source_line":914,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::mul_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21972,"parent_record_offset":21820,"ancestor_record_offset":18276,"inlinee":23596,"source_line":963,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":20380,"parent_record_offset":20344,"ancestor_record_offset":18276,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":20492,"parent_record_offset":20452,"ancestor_record_offset":18276,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":20656,"parent_record_offset":20452,"ancestor_record_offset":18276,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":20924,"parent_record_offset":20656,"ancestor_record_offset":18276,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":20520,"parent_record_offset":20492,"ancestor_record_offset":18276,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":20552,"parent_record_offset":20520,"ancestor_record_offset":18276,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":20452,"parent_record_offset":20416,"ancestor_record_offset":18276,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":20416,"parent_record_offset":20380,"ancestor_record_offset":18276,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":424,"record_offset":33936,"rva":5957600,"va":5374666720}
// 编译实例 RVAs: 0x5ae7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28423590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374666720,"raw_hex":"e8a6b5b101","size":5,"soft_fail":false,"target_va":5403091664,"va":5374668069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2861822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374666720,"raw_hex":"e8feaa2b00","size":5,"soft_fail":false,"target_va":5377529888,"va":5374668061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":35432,"parent_record_offset":33936,"ancestor_record_offset":33936,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":424,"record_offset":34488,"parent_record_offset":33936,"ancestor_record_offset":33936,"inlinee":17672,"source_line":1658,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":424,"record_offset":34328,"parent_record_offset":33936,"ancestor_record_offset":33936,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::composite
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":424,"record_offset":34660,"parent_record_offset":33936,"ancestor_record_offset":33936,"inlinee":23615,"source_line":153,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":35480,"parent_record_offset":35456,"ancestor_record_offset":33936,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":424,"record_offset":34512,"parent_record_offset":34488,"ancestor_record_offset":33936,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":424,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":33936,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":424,"record_offset":34428,"parent_record_offset":34404,"ancestor_record_offset":33936,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":34404,"parent_record_offset":34328,"ancestor_record_offset":33936,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":35456,"parent_record_offset":35432,"ancestor_record_offset":33936,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":424,"record_offset":34376,"parent_record_offset":34348,"ancestor_record_offset":33936,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":424,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":33936,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":424,"record_offset":34608,"parent_record_offset":34584,"ancestor_record_offset":33936,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":424,"record_offset":34348,"parent_record_offset":34328,"ancestor_record_offset":33936,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":424,"record_offset":34536,"parent_record_offset":34512,"ancestor_record_offset":33936,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::snapshot::try_load_snapshot_data::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":424,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":33936,"inlinee":24604,"source_line":284,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::as_float_grid
// 记录形式: inline_instance, inlinee_declaration; instances=64
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53A9
// 首条位置: {"module":364,"inlinee":21417}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::ws_to_px
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDB8
// 首条位置: {"module":1398,"inlinee":64952}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$15::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21916,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":64956,"source_line":1031,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$3::div_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21852,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":64954,"source_line":914,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::mul_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1398,"record_offset":21972,"parent_record_offset":21820,"ancestor_record_offset":21820,"inlinee":23596,"source_line":963,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":21820,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21996,"parent_record_offset":21972,"ancestor_record_offset":21820,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$392::div_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1398,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":21820,"inlinee":13202,"source_line":943,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::subscribe
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C39
// 首条位置: {"module":412,"inlinee":23609}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::composite
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C3F
// 首条位置: {"module":412,"inlinee":23615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::replace_composite
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x601A
// 首条位置: {"module":424,"inlinee":24602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":32712,"parent_record_offset":32612,"ancestor_record_offset":32612,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":424,"record_offset":32656,"parent_record_offset":32612,"ancestor_record_offset":32612,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":32760,"parent_record_offset":32736,"ancestor_record_offset":32612,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":424,"record_offset":32680,"parent_record_offset":32656,"ancestor_record_offset":32612,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":32736,"parent_record_offset":32712,"ancestor_record_offset":32612,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::WorldRaster::prepare_tile
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F65
// 首条位置: {"module":421,"inlinee":24421}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":24688,"parent_record_offset":23656,"ancestor_record_offset":23656,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24140,"parent_record_offset":23656,"ancestor_record_offset":23656,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24016,"parent_record_offset":23656,"ancestor_record_offset":23656,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":24744,"parent_record_offset":24688,"ancestor_record_offset":23656,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24472,"parent_record_offset":24272,"ancestor_record_offset":23656,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24072,"parent_record_offset":24016,"ancestor_record_offset":23656,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24200,"parent_record_offset":24140,"ancestor_record_offset":23656,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDCE
// 首条位置: {"module":1440,"inlinee":64974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":333572,"parent_record_offset":333552,"ancestor_record_offset":333552,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":333596,"parent_record_offset":333572,"ancestor_record_offset":333552,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":333620,"parent_record_offset":333596,"ancestor_record_offset":333552,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":333644,"parent_record_offset":333620,"ancestor_record_offset":333552,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10463
// 首条位置: {"module":1440,"inlinee":66659}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde::de::value::impl$26::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":333436,"parent_record_offset":333408,"ancestor_record_offset":333408,"inlinee":66649,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":333988,"parent_record_offset":333964,"ancestor_record_offset":333408,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":333964,"parent_record_offset":333892,"ancestor_record_offset":333408,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":333928,"parent_record_offset":333892,"ancestor_record_offset":333408,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":333892,"parent_record_offset":333860,"ancestor_record_offset":333408,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$28::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1440,"record_offset":333524,"parent_record_offset":333492,"ancestor_record_offset":333408,"inlinee":4445,"source_line":2477,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":333596,"parent_record_offset":333572,"ancestor_record_offset":333408,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":333804,"parent_record_offset":333492,"ancestor_record_offset":333408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":333620,"parent_record_offset":333596,"ancestor_record_offset":333408,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":333644,"parent_record_offset":333620,"ancestor_record_offset":333408,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":333572,"parent_record_offset":333552,"ancestor_record_offset":333408,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1440,"record_offset":333552,"parent_record_offset":333492,"ancestor_record_offset":333408,"inlinee":64974,"source_line":211,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":333492,"parent_record_offset":333464,"ancestor_record_offset":333408,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::value::impl$26::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/value.rs
//     evidence={"module":1440,"record_offset":333464,"parent_record_offset":333436,"ancestor_record_offset":333408,"inlinee":66650,"source_line":615,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDC4
// 首条位置: {"module":1398,"inlinee":64964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":28244,"parent_record_offset":28184,"ancestor_record_offset":28184,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":28280,"parent_record_offset":28184,"ancestor_record_offset":28184,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1398,"record_offset":28216,"parent_record_offset":28184,"ancestor_record_offset":28184,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDCF
// 首条位置: {"module":1440,"inlinee":64975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":546620,"parent_record_offset":546596,"ancestor_record_offset":546596,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":546644,"parent_record_offset":546620,"ancestor_record_offset":546596,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546668,"parent_record_offset":546644,"ancestor_record_offset":546596,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546692,"parent_record_offset":546668,"ancestor_record_offset":546596,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x104EA
// 首条位置: {"module":1440,"inlinee":66794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":546496,"parent_record_offset":546472,"ancestor_record_offset":546472,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":546892,"parent_record_offset":546868,"ancestor_record_offset":546472,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":546868,"parent_record_offset":546812,"ancestor_record_offset":546472,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":546840,"parent_record_offset":546812,"ancestor_record_offset":546472,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":546812,"parent_record_offset":546788,"ancestor_record_offset":546472,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":546644,"parent_record_offset":546620,"ancestor_record_offset":546472,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":546740,"parent_record_offset":546568,"ancestor_record_offset":546472,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546668,"parent_record_offset":546644,"ancestor_record_offset":546472,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546692,"parent_record_offset":546668,"ancestor_record_offset":546472,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":546620,"parent_record_offset":546596,"ancestor_record_offset":546472,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1440,"record_offset":546596,"parent_record_offset":546568,"ancestor_record_offset":546472,"inlinee":64975,"source_line":211,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":546568,"parent_record_offset":546544,"ancestor_record_offset":546472,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":546520,"parent_record_offset":546496,"ancestor_record_offset":546472,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":546544,"parent_record_offset":546520,"ancestor_record_offset":546472,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x104E8
// 首条位置: {"module":1440,"inlinee":66792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":547728,"parent_record_offset":544988,"ancestor_record_offset":544988,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":547764,"parent_record_offset":544988,"ancestor_record_offset":544988,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":545032,"parent_record_offset":544988,"ancestor_record_offset":544988,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":547176,"parent_record_offset":544988,"ancestor_record_offset":544988,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":545688,"parent_record_offset":545664,"ancestor_record_offset":544988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":545664,"parent_record_offset":545608,"ancestor_record_offset":544988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":546308,"parent_record_offset":546280,"ancestor_record_offset":544988,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":546368,"parent_record_offset":546308,"ancestor_record_offset":544988,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":545176,"parent_record_offset":545152,"ancestor_record_offset":544988,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":545228,"parent_record_offset":545200,"ancestor_record_offset":544988,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":545252,"parent_record_offset":545228,"ancestor_record_offset":544988,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":545636,"parent_record_offset":545608,"ancestor_record_offset":544988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":545820,"parent_record_offset":545796,"ancestor_record_offset":544988,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":545608,"parent_record_offset":545580,"ancestor_record_offset":544988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":545796,"parent_record_offset":545768,"ancestor_record_offset":544988,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":545768,"parent_record_offset":545736,"ancestor_record_offset":544988,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":546644,"parent_record_offset":546620,"ancestor_record_offset":544988,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":546400,"parent_record_offset":546308,"ancestor_record_offset":544988,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":547292,"parent_record_offset":547268,"ancestor_record_offset":544988,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":545968,"parent_record_offset":545940,"ancestor_record_offset":544988,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":545200,"parent_record_offset":545176,"ancestor_record_offset":544988,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":547268,"parent_record_offset":547232,"ancestor_record_offset":544988,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":545468,"parent_record_offset":545104,"ancestor_record_offset":544988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":545328,"parent_record_offset":545304,"ancestor_record_offset":544988,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":546336,"parent_record_offset":546308,"ancestor_record_offset":544988,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":547348,"parent_record_offset":547292,"ancestor_record_offset":544988,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546668,"parent_record_offset":546644,"ancestor_record_offset":544988,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":546692,"parent_record_offset":546668,"ancestor_record_offset":544988,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":545280,"parent_record_offset":545252,"ancestor_record_offset":544988,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":545304,"parent_record_offset":545280,"ancestor_record_offset":544988,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":546620,"parent_record_offset":546596,"ancestor_record_offset":544988,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1440,"record_offset":546596,"parent_record_offset":546568,"ancestor_record_offset":544988,"inlinee":64975,"source_line":211,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1440,"record_offset":546472,"parent_record_offset":546448,"ancestor_record_offset":544988,"inlinee":66794,"source_line":211,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":547836,"parent_record_offset":547812,"ancestor_record_offset":544988,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":547812,"parent_record_offset":547788,"ancestor_record_offset":544988,"inlinee":6871,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":545068,"parent_record_offset":545032,"ancestor_record_offset":544988,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":547204,"parent_record_offset":547176,"ancestor_record_offset":544988,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":546568,"parent_record_offset":546544,"ancestor_record_offset":544988,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":547408,"parent_record_offset":547384,"ancestor_record_offset":544988,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":546448,"parent_record_offset":545104,"ancestor_record_offset":544988,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":547684,"parent_record_offset":547660,"ancestor_record_offset":544988,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":545152,"parent_record_offset":545104,"ancestor_record_offset":544988,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":546520,"parent_record_offset":546496,"ancestor_record_offset":544988,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":546496,"parent_record_offset":546472,"ancestor_record_offset":544988,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":546544,"parent_record_offset":546520,"ancestor_record_offset":544988,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":545104,"parent_record_offset":545068,"ancestor_record_offset":544988,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":547232,"parent_record_offset":547204,"ancestor_record_offset":544988,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":27408,"rva":16986608,"va":5385695728}
// 编译实例 RVAs: 0x10331f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17264194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8426e0701","size":5,"soft_fail":false,"target_va":5402960384,"va":5385696185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9679883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8f54b6cff","size":5,"soft_fail":false,"target_va":5376016896,"va":5385696774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12947227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8e5703aff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385696934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140546de0
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11455300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8bc3451ff","size":5,"soft_fail":false,"target_va":5374242272,"va":5385697567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11452264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8984051ff","size":5,"soft_fail":false,"target_va":5374243632,"va":5385695891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9804133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e89b666aff","size":5,"soft_fail":false,"target_va":5375891872,"va":5385696000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e2f30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9766151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8f9fa6aff","size":5,"soft_fail":false,"target_va":5375930160,"va":5385696306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4b80
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9759578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8a6146bff","size":5,"soft_fail":false,"target_va":5375937408,"va":5385696981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f1360
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9708497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e82fdc6bff","size":5,"soft_fail":false,"target_va":5375988576,"va":5385697068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410d8480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t676133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e825510a00","size":5,"soft_fail":false,"target_va":5386372224,"va":5385696086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1410f4960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t791778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8e2140c00","size":5,"soft_fail":false,"target_va":5386488160,"va":5385696377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9330
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17260613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e845600701","size":5,"soft_fail":false,"target_va":5402956592,"va":5385695974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t17263593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695728,"raw_hex":"e8e96b0701","size":5,"soft_fail":false,"target_va":5402959760,"va":5385696162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] serde_json::value::de::impl$2::deserialize_enum
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1398,"record_offset":27588,"parent_record_offset":27408,"ancestor_record_offset":27408,"inlinee":25381,"source_line":255,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":27408,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1398,"record_offset":28620,"parent_record_offset":28528,"ancestor_record_offset":27408,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1398,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":27408,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1398,"record_offset":27784,"parent_record_offset":27756,"ancestor_record_offset":27408,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1398,"record_offset":27848,"parent_record_offset":27816,"ancestor_record_offset":27408,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1398,"record_offset":27908,"parent_record_offset":27876,"ancestor_record_offset":27408,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1398,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":27408,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":28556,"parent_record_offset":28528,"ancestor_record_offset":27408,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":28436,"parent_record_offset":28412,"ancestor_record_offset":27408,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1398,"record_offset":28528,"parent_record_offset":28504,"ancestor_record_offset":27408,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":28412,"parent_record_offset":28384,"ancestor_record_offset":27408,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1398,"record_offset":28384,"parent_record_offset":28352,"ancestor_record_offset":27408,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":29220,"parent_record_offset":29128,"ancestor_record_offset":27408,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1398,"record_offset":28584,"parent_record_offset":28556,"ancestor_record_offset":27408,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":27876,"parent_record_offset":27848,"ancestor_record_offset":27408,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":28156,"parent_record_offset":27588,"ancestor_record_offset":27408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1398,"record_offset":28016,"parent_record_offset":27992,"ancestor_record_offset":27408,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":29156,"parent_record_offset":29128,"ancestor_record_offset":27408,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":28244,"parent_record_offset":28184,"ancestor_record_offset":27408,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":28280,"parent_record_offset":28184,"ancestor_record_offset":27408,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":27968,"parent_record_offset":27936,"ancestor_record_offset":27408,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":27992,"parent_record_offset":27968,"ancestor_record_offset":27408,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1398,"record_offset":28184,"parent_record_offset":27588,"ancestor_record_offset":27408,"inlinee":64964,"source_line":211,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1398,"record_offset":28216,"parent_record_offset":28184,"ancestor_record_offset":27408,"inlinee":8752,"source_line":1999,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$17::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1398,"record_offset":27756,"parent_record_offset":27588,"ancestor_record_offset":27408,"inlinee":25383,"source_line":877,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1398,"record_offset":27816,"parent_record_offset":27588,"ancestor_record_offset":27408,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":114,"record_offset":2060,"rva":1723376,"va":5370432496}
// 编译实例 RVAs: 0x1a4bf0, 0x1033000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5593757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e89d5a5500","size":5,"soft_fail":false,"target_va":5376026640,"va":5370432878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct_variant
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5593249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e8a1585500","size":5,"soft_fail":false,"target_va":5376025840,"va":5370432586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401490d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-375839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e8e143faff","size":5,"soft_fail":false,"target_va":5370056912,"va":5370432746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1820898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e8e2c81b00","size":5,"soft_fail":false,"target_va":5372253808,"va":5370432905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140384a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1965190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e886fc1d00","size":5,"soft_fail":false,"target_va":5372398176,"va":5370432981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2317048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e8f85a2300","size":5,"soft_fail":false,"target_va":5372749696,"va":5370432643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2316925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e87d5a2300","size":5,"soft_fail":false,"target_va":5372749712,"va":5370432782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9803774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695232,"raw_hex":"e802686aff","size":5,"soft_fail":false,"target_va":5375891872,"va":5385695641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9745018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695232,"raw_hex":"e8864d6bff","size":5,"soft_fail":false,"target_va":5375950704,"va":5385695717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14108e9a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t374945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385695232,"raw_hex":"e8a1b80500","size":5,"soft_fail":false,"target_va":5386070432,"va":5385695482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32545205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370432496,"raw_hex":"e8b599f001","size":5,"soft_fail":false,"target_va":5402978144,"va":5370432934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":2900,"parent_record_offset":2060,"ancestor_record_offset":2060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":2252,"parent_record_offset":2060,"ancestor_record_offset":2060,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":114,"record_offset":2284,"parent_record_offset":2060,"ancestor_record_offset":2060,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::ser::impl$1::serialize_unit_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":2312,"parent_record_offset":2060,"ancestor_record_offset":2060,"inlinee":7826,"source_line":170,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":2060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":2628,"parent_record_offset":2600,"ancestor_record_offset":2060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":114,"record_offset":3060,"parent_record_offset":3004,"ancestor_record_offset":2060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":3188,"parent_record_offset":3160,"ancestor_record_offset":2060,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":114,"record_offset":3248,"parent_record_offset":3188,"ancestor_record_offset":2060,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":3032,"parent_record_offset":3004,"ancestor_record_offset":2060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":2060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":2060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":2748,"parent_record_offset":2572,"ancestor_record_offset":2060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":114,"record_offset":3004,"parent_record_offset":2976,"ancestor_record_offset":2060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":2508,"parent_record_offset":2480,"ancestor_record_offset":2060,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":2060,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":2424,"parent_record_offset":2396,"ancestor_record_offset":2060,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":2060,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":114,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":2060,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":114,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":2060,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":114,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":2060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":114,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":2060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":114,"record_offset":3280,"parent_record_offset":3188,"ancestor_record_offset":2060,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":114,"record_offset":2784,"parent_record_offset":2508,"ancestor_record_offset":2060,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":2928,"parent_record_offset":2900,"ancestor_record_offset":2060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":2060,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":114,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":114,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":2060,"inlinee":7827,"source_line":150,"archive":"_pdb/A/modules/00114.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$13::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFDB7
// 首条位置: {"module":1398,"inlinee":64951}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::clear_all<512>
// 记录形式: compiled_procedure; instances=1
// 源码行: 236
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":9236,"rva":16980256,"va":5385689376}
// 编译实例 RVAs: 0x1031920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2381882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8c6a7dbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385690021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17903850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8ea301101","size":5,"soft_fail":false,"target_va":5403593392,"va":5385689537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17288408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8d8cc0701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385690275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17290053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e845d30701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385690070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e892f5ffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385689433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17291136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e880d70701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385689515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9333323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8b59571ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385690230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9329446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8daa471ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385690001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9333265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385689376,"raw_hex":"e8ef9571ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385689404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-3158","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385689376,"raw_hex":"e9aaf3ffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385689729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":9984,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":11504,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":9396,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":10492,"parent_record_offset":9236,"ancestor_record_offset":9236,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":9236,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":9236,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":9236,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":9236,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":11860,"parent_record_offset":11828,"ancestor_record_offset":9236,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":12032,"parent_record_offset":11860,"ancestor_record_offset":9236,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":9236,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":9236,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":9620,"parent_record_offset":9524,"ancestor_record_offset":9236,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":9236,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":11088,"parent_record_offset":10628,"ancestor_record_offset":9236,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":11772,"parent_record_offset":11600,"ancestor_record_offset":9236,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":9236,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":12096,"parent_record_offset":11800,"ancestor_record_offset":9236,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":10448,"parent_record_offset":10424,"ancestor_record_offset":9236,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":9236,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":9236,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":10096,"parent_record_offset":10052,"ancestor_record_offset":9236,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":9236,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":12496,"parent_record_offset":11568,"ancestor_record_offset":9236,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":9556,"parent_record_offset":9524,"ancestor_record_offset":9236,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":9236,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":10816,"parent_record_offset":10792,"ancestor_record_offset":9236,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":10424,"parent_record_offset":10336,"ancestor_record_offset":9236,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":9236,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":10792,"parent_record_offset":10684,"ancestor_record_offset":9236,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":10052,"parent_record_offset":10016,"ancestor_record_offset":9236,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1398,"record_offset":10016,"parent_record_offset":9984,"ancestor_record_offset":9236,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":9524,"parent_record_offset":9496,"ancestor_record_offset":9236,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":10524,"parent_record_offset":10492,"ancestor_record_offset":9236,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":10656,"parent_record_offset":10628,"ancestor_record_offset":9236,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":10628,"parent_record_offset":10596,"ancestor_record_offset":9236,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9236,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":9236,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":9236,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":11120,"parent_record_offset":10596,"ancestor_record_offset":9236,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9720,"parent_record_offset":9688,"ancestor_record_offset":9236,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9880,"parent_record_offset":9720,"ancestor_record_offset":9236,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":9236,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9432,"parent_record_offset":9396,"ancestor_record_offset":9236,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9236,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9496,"parent_record_offset":9460,"ancestor_record_offset":9236,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":9236,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":10684,"parent_record_offset":10656,"ancestor_record_offset":9236,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":10932,"parent_record_offset":10684,"ancestor_record_offset":9236,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":10904,"parent_record_offset":10684,"ancestor_record_offset":9236,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":10736,"parent_record_offset":10684,"ancestor_record_offset":9236,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":11536,"parent_record_offset":11504,"ancestor_record_offset":9236,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":11636,"parent_record_offset":11600,"ancestor_record_offset":9236,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":11800,"parent_record_offset":11600,"ancestor_record_offset":9236,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":12068,"parent_record_offset":11800,"ancestor_record_offset":9236,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":11664,"parent_record_offset":11636,"ancestor_record_offset":9236,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":11696,"parent_record_offset":11664,"ancestor_record_offset":9236,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":11600,"parent_record_offset":11568,"ancestor_record_offset":9236,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":11568,"parent_record_offset":11536,"ancestor_record_offset":9236,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::copy_composite_to_scratch<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":12540,"rva":16981216,"va":5385690336}
// 编译实例 RVAs: 0x1031ce0, 0x1db9180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8e6a4dbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385690757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17902344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8082b1101","size":5,"soft_fail":false,"target_va":5403593360,"va":5385691011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17287609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8b9c90701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385691074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e80eeeffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385691165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17289317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e865d00701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385690806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17289399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8b7d00701","size":5,"soft_fail":false,"target_va":5402980464,"va":5385691060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8d2f1ffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385690393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t17290185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8c9d30701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385690466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9334063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8d19271ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385690970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-9330182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e8faa171ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385690737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9334225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385690336,"raw_hex":"e82f9271ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385690364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3ab327903ee31098 (.llvm.15844595411771089199)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1044518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399875968,"raw_hex":"e826f00f00","size":5,"soft_fail":false,"target_va":5400920912,"va":5399876389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tjmp\t-4463","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385690336,"raw_hex":"e991eeffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385691034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":15336,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":14304,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":12716,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":13292,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14688,"parent_record_offset":14660,"ancestor_record_offset":12540,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14740,"parent_record_offset":14716,"ancestor_record_offset":12540,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14716,"parent_record_offset":14688,"ancestor_record_offset":12540,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":12540,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14660,"parent_record_offset":14628,"ancestor_record_offset":12540,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14832,"parent_record_offset":14660,"ancestor_record_offset":12540,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":12540,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1398,"record_offset":15372,"parent_record_offset":15336,"ancestor_record_offset":12540,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":14764,"parent_record_offset":14740,"ancestor_record_offset":12540,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":12928,"parent_record_offset":12832,"ancestor_record_offset":12540,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":14524,"parent_record_offset":14496,"ancestor_record_offset":12540,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":13888,"parent_record_offset":13428,"ancestor_record_offset":12540,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":14572,"parent_record_offset":14400,"ancestor_record_offset":12540,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":13240,"parent_record_offset":13212,"ancestor_record_offset":12540,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":14896,"parent_record_offset":14600,"ancestor_record_offset":12540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":12540,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":13640,"parent_record_offset":13616,"ancestor_record_offset":12540,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":13848,"parent_record_offset":13816,"ancestor_record_offset":12540,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":14788,"parent_record_offset":14764,"ancestor_record_offset":12540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":15296,"parent_record_offset":14368,"ancestor_record_offset":12540,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":12864,"parent_record_offset":12832,"ancestor_record_offset":12540,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":12892,"parent_record_offset":12864,"ancestor_record_offset":12540,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":13616,"parent_record_offset":13592,"ancestor_record_offset":12540,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":13944,"parent_record_offset":13920,"ancestor_record_offset":12540,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":13592,"parent_record_offset":13484,"ancestor_record_offset":12540,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":12832,"parent_record_offset":12804,"ancestor_record_offset":12540,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":13324,"parent_record_offset":13292,"ancestor_record_offset":12540,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":13456,"parent_record_offset":13428,"ancestor_record_offset":12540,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":13428,"parent_record_offset":13396,"ancestor_record_offset":12540,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":13372,"parent_record_offset":13348,"ancestor_record_offset":12540,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":12540,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":12540,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":13920,"parent_record_offset":13396,"ancestor_record_offset":12540,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":13028,"parent_record_offset":12996,"ancestor_record_offset":12540,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":13188,"parent_record_offset":13028,"ancestor_record_offset":12540,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":12996,"parent_record_offset":12972,"ancestor_record_offset":12540,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":12748,"parent_record_offset":12716,"ancestor_record_offset":12540,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":13212,"parent_record_offset":13188,"ancestor_record_offset":12540,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":12804,"parent_record_offset":12772,"ancestor_record_offset":12540,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":12540,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":13484,"parent_record_offset":13456,"ancestor_record_offset":12540,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":13732,"parent_record_offset":13484,"ancestor_record_offset":12540,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":13704,"parent_record_offset":13484,"ancestor_record_offset":12540,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":13536,"parent_record_offset":13484,"ancestor_record_offset":12540,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":14336,"parent_record_offset":14304,"ancestor_record_offset":12540,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":14436,"parent_record_offset":14400,"ancestor_record_offset":12540,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":14600,"parent_record_offset":14400,"ancestor_record_offset":12540,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":14868,"parent_record_offset":14600,"ancestor_record_offset":12540,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":14464,"parent_record_offset":14436,"ancestor_record_offset":12540,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":14496,"parent_record_offset":14464,"ancestor_record_offset":12540,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":14400,"parent_record_offset":14368,"ancestor_record_offset":12540,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":14368,"parent_record_offset":14336,"ancestor_record_offset":12540,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::commit_scratch_to_composite<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":15408,"rva":16982064,"va":5385691184}
// 编译实例 RVAs: 0x1032030, 0x1db94d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2383466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e896a1dbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385691605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17901496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e8b8271101","size":5,"soft_fail":false,"target_va":5403593360,"va":5385691859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17286761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e869c60701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385691922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e8beeaffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385692013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17288469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e815cd0701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385691654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17288551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e867cd0701","size":5,"soft_fail":false,"target_va":5402980464,"va":5385691908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e882eeffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385691241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t17289337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e879d00701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385691314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9334911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e8818f71ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385691818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-9331030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e8aa9e71ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385691585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9335073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385691184,"raw_hex":"e8df8e71ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385691212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3ab327903ee31098 (.llvm.15844595411771089199)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1043670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399876816,"raw_hex":"e8d6ec0f00","size":5,"soft_fail":false,"target_va":5400920912,"va":5399877237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tjmp\t-5311","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385691184,"raw_hex":"e941ebffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385691882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":18204,"parent_record_offset":15408,"ancestor_record_offset":15408,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":17172,"parent_record_offset":15408,"ancestor_record_offset":15408,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":15584,"parent_record_offset":15408,"ancestor_record_offset":15408,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":16160,"parent_record_offset":15408,"ancestor_record_offset":15408,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17556,"parent_record_offset":17528,"ancestor_record_offset":15408,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17608,"parent_record_offset":17584,"ancestor_record_offset":15408,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17584,"parent_record_offset":17556,"ancestor_record_offset":15408,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17496,"parent_record_offset":17468,"ancestor_record_offset":15408,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17528,"parent_record_offset":17496,"ancestor_record_offset":15408,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17700,"parent_record_offset":17528,"ancestor_record_offset":15408,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":17812,"parent_record_offset":17788,"ancestor_record_offset":15408,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1398,"record_offset":18240,"parent_record_offset":18204,"ancestor_record_offset":15408,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":15408,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":15796,"parent_record_offset":15700,"ancestor_record_offset":15408,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":17392,"parent_record_offset":17364,"ancestor_record_offset":15408,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":16756,"parent_record_offset":16296,"ancestor_record_offset":15408,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":17440,"parent_record_offset":17268,"ancestor_record_offset":15408,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":16108,"parent_record_offset":16080,"ancestor_record_offset":15408,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":17764,"parent_record_offset":17468,"ancestor_record_offset":15408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":15408,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":16508,"parent_record_offset":16484,"ancestor_record_offset":15408,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":16716,"parent_record_offset":16684,"ancestor_record_offset":15408,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":17656,"parent_record_offset":17632,"ancestor_record_offset":15408,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":18164,"parent_record_offset":17236,"ancestor_record_offset":15408,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":15732,"parent_record_offset":15700,"ancestor_record_offset":15408,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":15760,"parent_record_offset":15732,"ancestor_record_offset":15408,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":15408,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":15408,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":16460,"parent_record_offset":16352,"ancestor_record_offset":15408,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":15700,"parent_record_offset":15672,"ancestor_record_offset":15408,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":16192,"parent_record_offset":16160,"ancestor_record_offset":15408,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":16324,"parent_record_offset":16296,"ancestor_record_offset":15408,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":16296,"parent_record_offset":16264,"ancestor_record_offset":15408,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15408,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":15408,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":15408,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":16788,"parent_record_offset":16264,"ancestor_record_offset":15408,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":15896,"parent_record_offset":15864,"ancestor_record_offset":15408,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":16056,"parent_record_offset":15896,"ancestor_record_offset":15408,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":15408,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":15616,"parent_record_offset":15584,"ancestor_record_offset":15408,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":15408,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":15672,"parent_record_offset":15640,"ancestor_record_offset":15408,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":15640,"parent_record_offset":15616,"ancestor_record_offset":15408,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":16352,"parent_record_offset":16324,"ancestor_record_offset":15408,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":16600,"parent_record_offset":16352,"ancestor_record_offset":15408,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":16572,"parent_record_offset":16352,"ancestor_record_offset":15408,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":16404,"parent_record_offset":16352,"ancestor_record_offset":15408,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":17204,"parent_record_offset":17172,"ancestor_record_offset":15408,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":17304,"parent_record_offset":17268,"ancestor_record_offset":15408,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":17468,"parent_record_offset":17268,"ancestor_record_offset":15408,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":17736,"parent_record_offset":17468,"ancestor_record_offset":15408,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":15408,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":17364,"parent_record_offset":17332,"ancestor_record_offset":15408,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":17268,"parent_record_offset":17236,"ancestor_record_offset":15408,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":17236,"parent_record_offset":17204,"ancestor_record_offset":15408,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1029D
// 首条位置: {"module":1423,"inlinee":66205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10298
// 首条位置: {"module":1423,"inlinee":66200}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 341
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10294
// 首条位置: {"module":1423,"inlinee":66196}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$6::saturating_add
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1423,"record_offset":2016,"parent_record_offset":1988,"ancestor_record_offset":1988,"inlinee":66197,"source_line":1499,"archive":"_pdb/A/modules/01423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::extend_dirty
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF37A
// 首条位置: {"module":1339,"inlinee":62330}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=15
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1339,"record_offset":10860,"parent_record_offset":10836,"ancestor_record_offset":10836,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01339.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::copy_to_staging<512>
// 记录形式: compiled_procedure; instances=2
// 源码行: 358
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1398,"record_offset":23188,"rva":16985008,"va":5385694128}
// 编译实例 RVAs: 0x1032bb0, 0x1db8500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405a1870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11081367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e869e956ff","size":5,"soft_fail":false,"target_va":5374613616,"va":5385694978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406cecb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9847416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e888bd69ff","size":5,"soft_fail":false,"target_va":5375847600,"va":5385695011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2386493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8c395dbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5385694632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ae0980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2981624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399872768,"raw_hex":"e80881d2ff","size":5,"soft_fail":false,"target_va":5396892032,"va":5399873651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141db7870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399872768,"raw_hex":"e819f0ffff","size":5,"soft_fail":false,"target_va":5399869552,"va":5399873618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17898356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8741b1101","size":5,"soft_fail":false,"target_va":5403593360,"va":5385694999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17899187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8b31e1101","size":5,"soft_fail":false,"target_va":5403593392,"va":5385694200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17283615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e81fba0701","size":5,"soft_fail":false,"target_va":5402978688,"va":5385695068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e851deffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385695194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17285442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e842c10701","size":5,"soft_fail":false,"target_va":5402980128,"va":5385694681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-7485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8c3e2ffff","size":5,"soft_fail":false,"target_va":5385686768,"va":5385694248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t17286330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8bac40701","size":5,"soft_fail":false,"target_va":5402980656,"va":5385694321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9337944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8a88371ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5385694851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-9334057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8d79271ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5385694612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-9338080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385694128,"raw_hex":"e8208371ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5385694219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h3ab327903ee31098 (.llvm.15844595411771089199)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1047635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399872768,"raw_hex":"e853fc0f00","size":5,"soft_fail":false,"target_va":5400920912,"va":5399873272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tjmp\t-8485","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5385694128,"raw_hex":"e9dbdeffff","size":5,"soft_fail":false,"target_va":5385686576,"va":5385695056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":23356,"parent_record_offset":23188,"ancestor_record_offset":23188,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1398,"record_offset":26108,"parent_record_offset":23188,"ancestor_record_offset":23188,"inlinee":64963,"source_line":13,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":25060,"parent_record_offset":23188,"ancestor_record_offset":23188,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":23472,"parent_record_offset":23188,"ancestor_record_offset":23188,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":24048,"parent_record_offset":23188,"ancestor_record_offset":23188,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25460,"parent_record_offset":25432,"ancestor_record_offset":23188,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25512,"parent_record_offset":25488,"ancestor_record_offset":23188,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25488,"parent_record_offset":25460,"ancestor_record_offset":23188,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25400,"parent_record_offset":25372,"ancestor_record_offset":23188,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25432,"parent_record_offset":25400,"ancestor_record_offset":23188,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25604,"parent_record_offset":25432,"ancestor_record_offset":23188,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1398,"record_offset":25716,"parent_record_offset":25692,"ancestor_record_offset":23188,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1398,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":23188,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1398,"record_offset":23684,"parent_record_offset":23588,"ancestor_record_offset":23188,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":25296,"parent_record_offset":25268,"ancestor_record_offset":23188,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":24644,"parent_record_offset":24184,"ancestor_record_offset":23188,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1398,"record_offset":25344,"parent_record_offset":25168,"ancestor_record_offset":23188,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1398,"record_offset":23996,"parent_record_offset":23968,"ancestor_record_offset":23188,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":25668,"parent_record_offset":25372,"ancestor_record_offset":23188,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":23188,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":23188,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1398,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":23188,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1398,"record_offset":25560,"parent_record_offset":25536,"ancestor_record_offset":23188,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1398,"record_offset":26068,"parent_record_offset":25132,"ancestor_record_offset":23188,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1398,"record_offset":23620,"parent_record_offset":23588,"ancestor_record_offset":23188,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1398,"record_offset":23648,"parent_record_offset":23620,"ancestor_record_offset":23188,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":24372,"parent_record_offset":24348,"ancestor_record_offset":23188,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":24700,"parent_record_offset":24676,"ancestor_record_offset":23188,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":24348,"parent_record_offset":24240,"ancestor_record_offset":23188,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1398,"record_offset":23404,"parent_record_offset":23380,"ancestor_record_offset":23188,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":1398,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":23188,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":23588,"parent_record_offset":23560,"ancestor_record_offset":23188,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1398,"record_offset":24080,"parent_record_offset":24048,"ancestor_record_offset":23188,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":24212,"parent_record_offset":24184,"ancestor_record_offset":23188,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1398,"record_offset":24184,"parent_record_offset":24152,"ancestor_record_offset":23188,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":24128,"parent_record_offset":24104,"ancestor_record_offset":23188,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":24152,"parent_record_offset":24128,"ancestor_record_offset":23188,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":24104,"parent_record_offset":24080,"ancestor_record_offset":23188,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1398,"record_offset":24676,"parent_record_offset":24152,"ancestor_record_offset":23188,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23784,"parent_record_offset":23752,"ancestor_record_offset":23188,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23944,"parent_record_offset":23784,"ancestor_record_offset":23188,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23752,"parent_record_offset":23728,"ancestor_record_offset":23188,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23504,"parent_record_offset":23472,"ancestor_record_offset":23188,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23968,"parent_record_offset":23944,"ancestor_record_offset":23188,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23560,"parent_record_offset":23528,"ancestor_record_offset":23188,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1398,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":23188,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":24240,"parent_record_offset":24212,"ancestor_record_offset":23188,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":24488,"parent_record_offset":24240,"ancestor_record_offset":23188,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":24460,"parent_record_offset":24240,"ancestor_record_offset":23188,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1398,"record_offset":24292,"parent_record_offset":24240,"ancestor_record_offset":23188,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":25096,"parent_record_offset":25060,"ancestor_record_offset":23188,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":25208,"parent_record_offset":25168,"ancestor_record_offset":23188,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":25372,"parent_record_offset":25168,"ancestor_record_offset":23188,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1398,"record_offset":25640,"parent_record_offset":25372,"ancestor_record_offset":23188,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1398,"record_offset":25236,"parent_record_offset":25208,"ancestor_record_offset":23188,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1398,"record_offset":25268,"parent_record_offset":25236,"ancestor_record_offset":23188,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":25168,"parent_record_offset":25132,"ancestor_record_offset":23188,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1398,"record_offset":25132,"parent_record_offset":25096,"ancestor_record_offset":23188,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::copy_to_staging::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F64
// 首条位置: {"module":421,"inlinee":24420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::finish
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25856,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":5839,"source_line":196,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] ahash::fallback_hash::impl$1::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25412,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":8799,"source_line":171,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":421,"record_offset":25956,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":26120,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":26052,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::prepare_tile
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":421,"record_offset":23656,"parent_record_offset":23528,"ancestor_record_offset":23528,"inlinee":24421,"source_line":175,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::convert::impl$0::read_u128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/convert.rs
//     evidence={"module":421,"record_offset":25576,"parent_record_offset":25412,"ancestor_record_offset":23528,"inlinee":8786,"source_line":110,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::large_update
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":25448,"parent_record_offset":25412,"ancestor_record_offset":23528,"inlinee":8781,"source_line":111,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::operations::folded_multiply
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/operations.rs
//     evidence={"module":421,"record_offset":25476,"parent_record_offset":25448,"ancestor_record_offset":23528,"inlinee":5828,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":421,"record_offset":24744,"parent_record_offset":24688,"ancestor_record_offset":23528,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25504,"parent_record_offset":25476,"ancestor_record_offset":23528,"inlinee":5830,"source_line":1703,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25540,"parent_record_offset":25448,"ancestor_record_offset":23528,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":421,"record_offset":25812,"parent_record_offset":25716,"ancestor_record_offset":23528,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":421,"record_offset":25672,"parent_record_offset":25648,"ancestor_record_offset":23528,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":24688,"parent_record_offset":23656,"ancestor_record_offset":23528,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":23528,"inlinee":6165,"source_line":1886,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25624,"parent_record_offset":25600,"ancestor_record_offset":23528,"inlinee":6166,"source_line":2086,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":421,"record_offset":25648,"parent_record_offset":25624,"ancestor_record_offset":23528,"inlinee":6167,"source_line":1973,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24140,"parent_record_offset":23656,"ancestor_record_offset":23528,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24016,"parent_record_offset":23656,"ancestor_record_offset":23528,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":23528,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24472,"parent_record_offset":24272,"ancestor_record_offset":23528,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24072,"parent_record_offset":24016,"ancestor_record_offset":23528,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":421,"record_offset":24200,"parent_record_offset":24140,"ancestor_record_offset":23528,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::needs_texture_upload
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27A7
// 首条位置: {"module":136,"inlinee":10151}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::on_uploaded_to_texture
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27A8
// 首条位置: {"module":136,"inlinee":10152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::needs_texture_processing
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 493
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x649D
// 首条位置: {"module":448,"inlinee":25757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$1::on_texture_processed
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 497
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x649E
// 首条位置: {"module":448,"inlinee":25758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::utils::dirty_tiles::DirtyTiles::clear
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":448,"record_offset":36124,"parent_record_offset":36096,"ancestor_record_offset":36096,"inlinee":25759,"source_line":132,"archive":"_pdb/A/modules/00448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::build_tile_hasher
// 记录形式: compiled_procedure; instances=1
// 源码行: 503
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":421,"record_offset":5132,"rva":5899184,"va":5374608304}
// 编译实例 RVAs: 0x5a03b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ahash::random_state::impl$5::build_hasher
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/random_state.rs
//     evidence={"module":421,"record_offset":5272,"parent_record_offset":5132,"ancestor_record_offset":5132,"inlinee":24383,"source_line":418,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   [pdb_inline_transitive] ahash::fallback_hash::AHasher::from_random_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ahash","version":"0.8.11","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.11/src/fallback_hash.rs
//     evidence={"module":421,"record_offset":5292,"parent_record_offset":5272,"ancestor_record_offset":5132,"inlinee":6685,"source_line":57,"archive":"_pdb/A/modules/00421.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster::impl$3::load_index_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=287
// 源码行: 524
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53B8
// 首条位置: {"module":364,"inlinee":21432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::impl$15::deref_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 576
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":33500,"rva":4440720,"va":5373149840}
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
// 函数符号: country_core::resources::world_raster_defs::impl$24::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 576
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":33640,"rva":4440720,"va":5373149840}
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
// 函数符号: country_core::resources::world_raster_defs::GardenRaster::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 580
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B40
// 首条位置: {"module":406,"inlinee":23360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::PathRaster::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 580
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C59
// 首条位置: {"module":412,"record_offset":32148,"rva":5744608,"va":5374453728}
// 编译实例 RVAs: 0x57a7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141031320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11234087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453728,"raw_hex":"e8276bab00","size":5,"soft_fail":false,"target_va":5385687840,"va":5374453748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29139590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453728,"raw_hex":"e886a2bc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374453765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":33920,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::get_param
// 记录形式: compiled_procedure; instances=2
// 源码行: 593, 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":34192,"rva":5745200,"va":5374454320}
// 编译实例 RVAs: 0x57aa30, 0x57ac90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28524122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374454320,"raw_hex":"e85a3eb301","size":5,"soft_fail":false,"target_va":5402978944,"va":5374454817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$85::get_param
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":34340,"parent_record_offset":34192,"ancestor_record_offset":34192,"inlinee":5144,"source_line":1466,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":36228,"parent_record_offset":36204,"ancestor_record_offset":34192,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":36204,"parent_record_offset":36176,"ancestor_record_offset":34192,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":412,"record_offset":34856,"parent_record_offset":34820,"ancestor_record_offset":34192,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":34792,"parent_record_offset":34404,"ancestor_record_offset":34192,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":34820,"parent_record_offset":34792,"ancestor_record_offset":34192,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":34436,"parent_record_offset":34404,"ancestor_record_offset":34192,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":412,"record_offset":34884,"parent_record_offset":34820,"ancestor_record_offset":34192,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":34484,"parent_record_offset":34460,"ancestor_record_offset":34192,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":34460,"parent_record_offset":34436,"ancestor_record_offset":34192,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":34576,"parent_record_offset":34512,"ancestor_record_offset":34192,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":412,"record_offset":34660,"parent_record_offset":34636,"ancestor_record_offset":34192,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$4::get_param
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":34368,"parent_record_offset":34340,"ancestor_record_offset":34192,"inlinee":4356,"source_line":525,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$4::get_param::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":34192,"inlinee":4403,"source_line":533,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":36736,"parent_record_offset":36708,"ancestor_record_offset":36560,"inlinee":4382,"source_line":620,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$7::get_param::closure$0
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":37500,"parent_record_offset":37476,"ancestor_record_offset":36560,"inlinee":4407,"source_line":628,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":412,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":36560,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":37360,"parent_record_offset":36772,"ancestor_record_offset":36560,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":36772,"parent_record_offset":36736,"ancestor_record_offset":36560,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_with_ticks
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":412,"record_offset":34404,"parent_record_offset":34368,"ancestor_record_offset":34192,"inlinee":4357,"source_line":533,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":412,"record_offset":34980,"parent_record_offset":34952,"ancestor_record_offset":34192,"inlinee":4405,"source_line":699,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":34596,"parent_record_offset":34576,"ancestor_record_offset":34192,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":34636,"parent_record_offset":34460,"ancestor_record_offset":34192,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":34928,"parent_record_offset":34368,"ancestor_record_offset":34192,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":34740,"parent_record_offset":34716,"ancestor_record_offset":34192,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":34512,"parent_record_offset":34484,"ancestor_record_offset":34192,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":34548,"parent_record_offset":34512,"ancestor_record_offset":34192,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":34716,"parent_record_offset":34684,"ancestor_record_offset":34192,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":37408,"parent_record_offset":37384,"ancestor_record_offset":36560,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":37432,"parent_record_offset":37408,"ancestor_record_offset":36560,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":412,"record_offset":34684,"parent_record_offset":34660,"ancestor_record_offset":34192,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::init_state
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 593, 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C63
// 首条位置: {"module":412,"record_offset":36376,"rva":5745712,"va":5374454832}
// 编译实例 RVAs: 0x57ac30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..change_detection..ResMut$LT$T$GT$$u20$as$u20$bevy_ecs..system..system_param..SystemParam$GT$::init_state
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t10719953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374454832,"raw_hex":"e8d192a300","size":5,"soft_fail":false,"target_va":5385174816,"va":5374454858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$85::init_state
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":412,"record_offset":36528,"parent_record_offset":36376,"ancestor_record_offset":36376,"inlinee":5139,"source_line":1443,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::new_archetype
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":33776,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::_::impl$0::queue
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":34056,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5C5B
// 首条位置: {"module":1438,"inlinee":23643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::composite
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1438,"record_offset":31704,"parent_record_offset":31680,"ancestor_record_offset":31680,"inlinee":23615,"source_line":153,"archive":"_pdb/A/modules/01438.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":424,"record_offset":31932,"rva":5956672,"va":5374665792}
// 编译实例 RVAs: 0x5ae440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28425458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e8f2bcb101","size":5,"soft_fail":false,"target_va":5403091664,"va":5374666201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28927473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e8f165b901","size":5,"soft_fail":false,"target_va":5403593360,"va":5374665882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28927485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e8fd65b901","size":5,"soft_fail":false,"target_va":5403593392,"va":5374665902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2863690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e84ab22b00","size":5,"soft_fail":false,"target_va":5377529888,"va":5374666193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2697727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e8ff292900","size":5,"soft_fail":false,"target_va":5377364336,"va":5374666604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28312306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e8f202b001","size":5,"soft_fail":false,"target_va":5402978880,"va":5374666569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t28313878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374665792,"raw_hex":"e81609b001","size":5,"soft_fail":false,"target_va":5402980128,"va":5374666245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":424,"record_offset":32556,"parent_record_offset":31932,"ancestor_record_offset":31932,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":424,"record_offset":32320,"parent_record_offset":31932,"ancestor_record_offset":31932,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":33904,"parent_record_offset":31932,"ancestor_record_offset":31932,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":424,"record_offset":33712,"parent_record_offset":31932,"ancestor_record_offset":31932,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::replace_composite
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":424,"record_offset":32612,"parent_record_offset":31932,"ancestor_record_offset":31932,"inlinee":24602,"source_line":157,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":32712,"parent_record_offset":32612,"ancestor_record_offset":31932,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":424,"record_offset":32760,"parent_record_offset":32736,"ancestor_record_offset":31932,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":424,"record_offset":32580,"parent_record_offset":32556,"ancestor_record_offset":31932,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":424,"record_offset":32680,"parent_record_offset":32656,"ancestor_record_offset":31932,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":424,"record_offset":32496,"parent_record_offset":32472,"ancestor_record_offset":31932,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":32472,"parent_record_offset":32320,"ancestor_record_offset":31932,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":424,"record_offset":32736,"parent_record_offset":32712,"ancestor_record_offset":31932,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":424,"record_offset":33744,"parent_record_offset":33712,"ancestor_record_offset":31932,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":424,"record_offset":32436,"parent_record_offset":32412,"ancestor_record_offset":31932,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":424,"record_offset":32384,"parent_record_offset":32348,"ancestor_record_offset":31932,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":424,"record_offset":32656,"parent_record_offset":32612,"ancestor_record_offset":31932,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":424,"record_offset":32412,"parent_record_offset":32348,"ancestor_record_offset":31932,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":424,"record_offset":32348,"parent_record_offset":32320,"ancestor_record_offset":31932,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::snapshot::init_game_state_from_snapshot::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":424,"record_offset":32520,"parent_record_offset":32496,"ancestor_record_offset":31932,"inlinee":24601,"source_line":380,"archive":"_pdb/A/modules/00424.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 记录形式: compiled_procedure; instances=1
// 源码行: 650
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":32284,"rva":5744672,"va":5374453792}
// 编译实例 RVAs: 0x57a820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140667370
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t969510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453792,"raw_hex":"e826cb0e00","size":5,"soft_fail":false,"target_va":5375423344,"va":5374453829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141032bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11240184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453792,"raw_hex":"e8f882ab00","size":5,"soft_fail":false,"target_va":5385694128,"va":5374453939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28504258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453792,"raw_hex":"e8c2f0b201","size":5,"soft_fail":false,"target_va":5402958528,"va":5374454265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29139444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453792,"raw_hex":"e8f4a1bc01","size":5,"soft_fail":false,"target_va":5403593344,"va":5374453895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28524655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374453792,"raw_hex":"e86f40b301","size":5,"soft_fail":false,"target_va":5402978880,"va":5374454220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141032bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: world_raster.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t11239915","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374453792,"raw_hex":"e9eb81ab00","size":5,"soft_fail":false,"target_va":5385694128,"va":5374454208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374453792,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5374453915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":412,"record_offset":32464,"parent_record_offset":32284,"ancestor_record_offset":32284,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":412,"record_offset":32740,"parent_record_offset":32284,"ancestor_record_offset":32284,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":412,"record_offset":32516,"parent_record_offset":32284,"ancestor_record_offset":32284,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data_mut
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":412,"record_offset":32768,"parent_record_offset":32284,"ancestor_record_offset":32284,"inlinee":23616,"source_line":296,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":412,"record_offset":32488,"parent_record_offset":32464,"ancestor_record_offset":32284,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":32572,"parent_record_offset":32540,"ancestor_record_offset":32284,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":412,"record_offset":32656,"parent_record_offset":32628,"ancestor_record_offset":32284,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":412,"record_offset":32684,"parent_record_offset":32656,"ancestor_record_offset":32284,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":32600,"parent_record_offset":32572,"ancestor_record_offset":32284,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":412,"record_offset":32540,"parent_record_offset":32516,"ancestor_record_offset":32284,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00412.jsonl.gz"}
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
// 完整结构及原始类型记录: world_raster.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::StrokeBlendMode
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::resources::world_raster::StrokeBlendMode>", "constructor": "enum2$", "arguments": ["country_core::resources::world_raster::StrokeBlendMode"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<country_core::resources::world_raster::StrokeBlendMode>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::EventReader", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoSystemConfigs::in_set", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)", "void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::EventRegistry::register_event<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::EventRegistry::register_event", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::event::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>", "constructor": "bevy_ecs::event::Events", "arguments": ["country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$4::init_state<bevy_ecs::event::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$4::init_state", "arguments": ["bevy_ecs::event::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<bevy_ecs::event::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd> >", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["bevy_ecs::event::Events<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::world_raster::WorldRaster
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster<512>", "constructor": "country_core::resources::world_raster::WorldRaster", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::world_raster::WorldRaster<512> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::world_raster::WorldRaster<512>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$1", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$1", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster::closure_env$1", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_stroke", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >", "constructor": "country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > > >", "constructor": "conservative_line_raster::conservative_plot_line", "arguments": ["country_core::utils::dirty_tiles::impl$0::rasterize_world_space_stroke::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_path::replay_path::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_water::replay_water::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>", "constructor": "country_core::systems::history::replay_raster::replay_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::history::replay_garden::replay_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::GardenRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::garden::draw_garden::draw_garden::closure_env$0", "country_core::systems::garden::draw_garden::draw_garden::closure_env$1", "country_core::systems::garden::draw_garden::draw_garden::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::PathRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "country_core::systems::paths::draw_path::draw_path::closure_env$0", "country_core::systems::paths::draw_path::draw_path::closure_env$1", "country_core::systems::paths::draw_path::draw_path::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >", "ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec2::Vec2,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > > >", "constructor": "itertools::tuple_impl::impl$26::collect_from_iter_no_buf", "arguments": ["glam::f32::vec2::Vec2", "core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec2::Vec2>,core::iter::sources::once::Once<glam::f32::vec2::Vec2> >,ref_mut$<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::history::replay_raster::replay_raster::closure_env$1<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::world_raster::WorldRaster::new<512>", "constructor": "country_core::resources::world_raster::WorldRaster::new", "arguments": ["512"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3>", "constructor": "country_core::systems::world_raster::draw_raster", "arguments": ["country_core::resources::world_raster_defs::WaterRasterStagingSsbo", "country_core::resources::world_raster::WorldRaster<512>", "system_water::draw_water::draw_water::closure_env$1", "system_water::draw_water::draw_water::closure_env$2", "system_water::draw_water::draw_water::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: world_raster.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/world_raster.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: world_raster.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 298; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::intersect_meshes::MeshIntersectionGraph>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::BirdNestCandidateLocations>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::as_float_grid
// 信息 ID: 63a4035ec93917e5d188b417
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 41 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=6056; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::garden_audio::garden_audio_system; module=392, inline_record=8876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::decorator_audio::decorator_audio_system; module=409, inline_record=14972; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::ui_edit_terrain::ui_edit_terrain; module=430, inline_record=40148; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain; module=452, inline_record=28628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=499, inline_record=19292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=499, inline_record=19320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::firstperson_steps::firstperson_steps; module=499, inline_record=19348; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::composite
// 信息 ID: 1cae63684f8e251a10c85971
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=34660; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=35028; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; module=424, inline_record=35404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31732; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::new<512>
// 信息 ID: f04194d721dceab867d0b981
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::WorldRaster::new -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::garden::impl$0::build @ 0x561052 -> 0x1031320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::PathRaster::new @ 0x57a7f4 -> 0x1031320 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::paths::impl$0::build @ 0x5c8261 -> 0x1031320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::prepare_tile
// 信息 ID: da7f6cb5ae2356991c5616f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; module=421, inline_record=23656; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; module=4205, inline_record=1944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::replace_composite
// 信息 ID: 11b3aa98e7e070cad422b831
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=32612; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=33100; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >; module=424, inline_record=33524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::subscribe
// 信息 ID: aa3285d38fd879f768184916
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4cd2a4be13acb499e48dca4a: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)>; module=413, inline_record=9916; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> >; module=1325, inline_record=2341460; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids; module=3515, inline_record=6748; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids; module=3515, inline_record=7204; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids; module=3515, inline_record=7660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::WorldRaster::ws_to_px
// 信息 ID: cb627db36060919b33795a86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=1398, inline_record=21820; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=1398, inline_record=22364; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=4206, inline_record=6744; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >; module=4206, inline_record=7288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 650362959d24df1bd54c450c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1440, inline_record=333552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 2fd7d57ae6336942780eca1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::variant_seed<core::marker::PhantomData<country_core::resources::world_raster::_::impl$0::deserialize::__Field> >; module=1440, inline_record=333408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: a07ff44734504198035b2333
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1398, inline_record=28184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: 027e3922ec8b4eb6e22dfa57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=546596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 4e7e4da810d37a77019b4887
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=546472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: 8df35306916b247d151c967c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$5::struct_variant<country_core::resources::world_raster::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=1440, inline_record=544988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: d4dc3c27325f7192221f85f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor> @ 0x10b6d77 -> 0x10331f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: 4cc8cccfecbef8bc4b4e1618
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x1525db -> 0x1a4bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$8::serialize_field<enum2$<country_core::resources::world_raster::StrokeBlendMode> > @ 0x109d8fb -> 0x1033000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::build_tile_hasher
// 信息 ID: b3fa5f70c39393d290bc87cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> > @ 0x1db7d43 -> 0x5a03b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$13::fmt
// 信息 ID: 8619efb7e441b335f00b012f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<enum2$<country_core::resources::world_raster::StrokeBlendMode> >; module=1398, inline_record=420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::clear_all<512>
// 信息 ID: b0ac0f1dfffd2c4e6b1d3593
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::clear_all -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67d79e -> 0x1031920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67d7c6 -> 0x1031920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67d7f4 -> 0x1031920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::clear_buffer<512>
// 信息 ID: 60b7615e4d2a5d5bd0d7be0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::clear_buffer -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec78f4 -> 0x10315f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xec9ef4 -> 0x10315f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc634 -> 0x10315f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::commit_scratch_to_composite<512>
// 信息 ID: 7586e9ad8a8d3de75c516466
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::commit_scratch_to_composite -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec90f9 -> 0x1032030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecb82b -> 0x1032030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecde39 -> 0x1032030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecef55 -> 0x1032030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0245 -> 0x1032030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e882 -> 0x1db94d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::copy_composite_to_scratch<512>
// 信息 ID: 262ed0145f4b741b8eeacf8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::copy_composite_to_scratch -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8f17 -> 0x1031ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecb5af -> 0x1031ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecdc57 -> 0x1031ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecfab3 -> 0x1031ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0da3 -> 0x1031ce0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e653 -> 0x1db9180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::copy_to_staging::closure$0
// 信息 ID: 2b8574e6a0bf531d5abaf45e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$1::copy_to_staging", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0 <- country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; module=421, inline_record=23528; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::DirtyTiles::retain<country_core::resources::world_raster::impl$1::copy_to_staging::closure_env$0<512> >; module=4205, inline_record=1812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::copy_to_staging<512>
// 信息 ID: 2ec37756e89a7b1880caebb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::world_raster::impl$1::copy_to_staging -> ["512"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x57a8b3 -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x57a93d -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging @ 0x57a9c0 -> 0x1032bb0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7e7a -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8fda -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca50f -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecb672 -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xeccbba -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecdd1a -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecfb6f -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecefed -> 0x1032bb0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0e5f -> 0x1032bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed02dd -> 0x1032bb0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e718 -> 0x1db8500 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e91a -> 0x1db8500 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::extend_dirty
// 信息 ID: a1eb43de4d024e38edc94105
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=10836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17452; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2>; module=1339, inline_record=17508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=27708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=34308; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2>; module=1339, inline_record=34364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=1339, inline_record=44568; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2>; module=1339, inline_record=51184; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::needs_texture_processing
// 信息 ID: d717bdccaa0971427c533389
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=448, inline_record=36012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=498, inline_record=21500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::needs_texture_upload
// 信息 ID: f48e6a8e08c340128344bd20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=136, inline_record=3080; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=136, inline_record=6284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=136, inline_record=9488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::on_texture_processed
// 信息 ID: 7e231015d67a9175ea41fae9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu; module=448, inline_record=36096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::draw_path_mask::draw_path_mask_compute; module=459, inline_record=60628; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu; module=498, inline_record=21584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::on_uploaded_to_texture
// 信息 ID: 9e85dfe8650773e51868e7f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::GardenRaster>; module=136, inline_record=3164; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::WaterRaster>; module=136, inline_record=6368; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture<country_core::resources::world_raster_defs::PathRaster>; module=136, inline_record=9572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$0
// 信息 ID: 2cb2adbe9201d1cb4ef85948
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::world_raster::impl$1::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 34 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=9068; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=9384; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=9556; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=11368; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=11684; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=11856; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=18292; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$0<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=18608; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$1
// 信息 ID: 258be5acf13c8da1b21121ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::world_raster::impl$1::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 34 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=4352; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=4668; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$2<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$3<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=4840; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=13640; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=13956; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$4<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$5<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=14128; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=15940; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$1<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=16256; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::rasterize_stroke::closure$2
// 信息 ID: 80cf5a1057d45122e6be2386
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::world_raster::impl$1::rasterize_stroke", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2 <- zeno::raster::Rasterizer::rasterize_write<zeno::raster::HeapStorage,zeno::mask::render::closure_env$0<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$1<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 48 条；闭包环境类型引用 34 条。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=1988; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=2336; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$11<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$12<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=2536; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=6704; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=7052; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$7<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$8<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=7252; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=27620; 不是独立调用边。
// [PDB 内联宿主] zeno::raster::Rasterizer::rasterize_write<zeno::raster::AdaptiveStorage,zeno::mask::render::closure_env$9<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > >,zeno::mask::render::closure_env$10<ref$<slice2$<enum2$<zeno::command::Command> > >,country_core::resources::world_raster::impl$1::rasterize_stroke::closure_env$2<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> > > >; module=1423, inline_record=27968; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$1::rasterize_stroke<512,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0> >
// 信息 ID: 2e81f04bd98f6bfb8ef920a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>", "country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster::impl$1::rasterize_stroke -> ["512", "core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::raster_stroke::impl$1::iter_points_xz::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec79a7 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec7a0e -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_path::replay_path::closure_env$2> @ 0xec8f58 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xec9fa7 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xeca00e -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_water::replay_water::closure_env$2> @ 0xecb5f0 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc6e7 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecc74e -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_raster::replay_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::history::replay_garden::replay_garden::closure_env$2> @ 0xecdc98 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env$0,country_core::systems::garden::draw_garden::draw_garden::closure_env$1,country_core::systems::garden::draw_garden::draw_garden::closure_env$2> @ 0xecfaf2 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::PathRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::paths::draw_path::draw_path::closure_env$0,country_core::systems::paths::draw_path::draw_path::closure_env$1,country_core::systems::paths::draw_path::draw_path::closure_env$2> @ 0xed0de2 -> 0x1032380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::world_raster::draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env$1,system_water::draw_water::draw_water::closure_env$2,system_water::draw_water::draw_water::closure_env$3> @ 0x1a8e699 -> 0x1db8950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster::impl$3::load_index_unchecked
// 信息 ID: 144786c7940ab30b26c5ceb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 260 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11700; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11780; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11856; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=11932; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13736; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13816; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13892; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=13968; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterRead::read
// 信息 ID: 7702e6087ec782e26b8f6650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::capture_game_snapshot; module=1438, inline_record=31680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::upload_to_staging
// 信息 ID: e13fe255a08dd058c6cdc265
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::world_raster::upload_world_raster_buffers_to_staging @ 0x6b1262 -> 0x57a820 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >
// 信息 ID: 1243bf4396e4f4fc21ab74e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >", "country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x10a9c92 -> 0x5ae7e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::AllWorldRasterWrite::write<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0> >
// 信息 ID: 2cf587307d1b3859b9c418c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >", "country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0"]; 字段布局 = unknown。
// [类型关系] country_core::resources::world_raster_defs::AllWorldRasterWrite::write -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> > >,country_core::systems::save::snapshot::init_game_state_from_snapshot::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::init_game_state_from_snapshot @ 0x10ab5f2 -> 0x5ae440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::GardenRaster::new
// 信息 ID: 552bd4f7931383da7c16ece3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::garden::impl$0::build; module=406, inline_record=78092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::PathRaster::new
// 信息 ID: 06126a6f15fa0eb7942301f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_water::impl$0::build @ 0x40e217 -> 0x57a7e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::paths::impl$0::build; module=431, inline_record=112648; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::apply
// 信息 ID: c3c1d5798085335ae871128c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::get_param
// 信息 ID: 665fd40175662de38fcc5c9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 79137c6f444377e5cdc6b8cd: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] d719c60c87d023445401aae9: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> @ 0x672f3a -> 0x57aa30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)> > @ 0xe5110e -> 0x57aa30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x66220 -> 0x57ac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> > @ 0xd4e80 -> 0x57ac90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::snapshot::_::impl$0::get_param @ 0x10ac102 -> 0x57ac90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::init_state
// 信息 ID: 5ed35e5800e9f1796d0b15c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 79137c6f444377e5cdc6b8cd: void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)
// [泛型宿主/闭包/直接引用方参数候选] d719c60c87d023445401aae9: void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::initialize<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x65e41 -> 0x57ac30 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::initialize<void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >),void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> > >,country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::event::EventReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::event::EventReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)>; module=478, inline_record=5116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::_::impl$0::init_state; module=1438, inline_record=70956; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::new_archetype
// 信息 ID: 3e7025e52a5db7021adb1c66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::_::impl$0::queue
// 信息 ID: 5c6e62f730e5ff054b1258bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::world_raster_defs::impl$15::deref_mut
// 信息 ID: 96d1dd0db1c052f6794897b7
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
// 函数: country_core::resources::world_raster_defs::impl$24::deref
// 信息 ID: b130be6f1807330b51171520
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

// === PDB ENRICHMENT V1 END ===
