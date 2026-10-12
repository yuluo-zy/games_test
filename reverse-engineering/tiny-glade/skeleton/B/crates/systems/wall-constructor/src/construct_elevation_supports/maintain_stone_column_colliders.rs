// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a2c858f75826c1e4db6f5fbe2d43e6c2eb13373fe96511dac07ec2545a86ff24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: maintain_stone_column_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 412, 1415, 1417, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":412,"record_offset":1660,"rva":7907040,"va":5376616160}
// 编译实例 RVAs: 0x78a6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1941178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e84661e2ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376617509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1536527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8f18de8ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376618010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1535671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e84991e8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376617170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4004583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8e71a3d00","size":5,"soft_fail":false,"target_va":5380621856,"va":5376617268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fc410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10951002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e85a19a700","size":5,"soft_fail":false,"target_va":5387568144,"va":5376617137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10952518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8461fa700","size":5,"soft_fail":false,"target_va":5387572000,"va":5376619477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141203fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10982452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e83494a700","size":5,"soft_fail":false,"target_va":5387599840,"va":5376617383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141204710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10983874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8c299a700","size":5,"soft_fail":false,"target_va":5387601680,"va":5376617801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141245220
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11249425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e811a7ab00","size":5,"soft_fail":false,"target_va":5387866656,"va":5376617226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412453c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11248504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e878a3ab00","size":5,"soft_fail":false,"target_va":5387867072,"va":5376618563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: maintain_stone_column_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35228705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8218c1902","size":5,"soft_fail":false,"target_va":5411847168,"va":5376618458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34922072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e858de1402","size":5,"soft_fail":false,"target_va":5411540992,"va":5376618915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2263113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8b777ddff","size":5,"soft_fail":false,"target_va":5374355568,"va":5376618676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34922014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e81ede1402","size":5,"soft_fail":false,"target_va":5411540963,"va":5376618944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34922679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8b7e01402","size":5,"soft_fail":false,"target_va":5411541584,"va":5376618900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34922947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8c3e11402","size":5,"soft_fail":false,"target_va":5411541776,"va":5376618824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e868fbffff","size":5,"soft_fail":false,"target_va":5376615088,"va":5376616259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34927019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8abf11402","size":5,"soft_fail":false,"target_va":5411543360,"va":5376616336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMesh::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1325838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e80e3b1400","size":5,"soft_fail":false,"target_va":5377944336,"va":5376618493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1326838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8f63e1400","size":5,"soft_fail":false,"target_va":5377945152,"va":5376618309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4768765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8fdc34800","size":5,"soft_fail":false,"target_va":5381385536,"va":5376616766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4773954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e842d84800","size":5,"soft_fail":false,"target_va":5381390640,"va":5376616681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4767707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8dbbf4800","size":5,"soft_fail":false,"target_va":5381386480,"va":5376618768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4768470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e8d6c24800","size":5,"soft_fail":false,"target_va":5381384704,"va":5376616229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35021925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e865641602","size":5,"soft_fail":false,"target_va":5411640768,"va":5376618838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34918211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376616160,"raw_hex":"e843cf1402","size":5,"soft_fail":false,"target_va":5411537104,"va":5376618888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":412,"record_offset":9500,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6196,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":7760,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":5436,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6572,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":412,"record_offset":6660,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":412,"record_offset":5380,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":412,"record_offset":5040,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":412,"record_offset":4344,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":412,"record_offset":10048,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":412,"record_offset":6488,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":412,"record_offset":11456,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":5492,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":7788,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":3360,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":1936,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":2384,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":412,"record_offset":6788,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":412,"record_offset":5464,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":1660,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":1660,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":1660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":9572,"parent_record_offset":9536,"ancestor_record_offset":1660,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":8416,"parent_record_offset":8236,"ancestor_record_offset":1660,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":412,"record_offset":5680,"parent_record_offset":5588,"ancestor_record_offset":1660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":412,"record_offset":9536,"parent_record_offset":9500,"ancestor_record_offset":1660,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":1660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":1660,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":1660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":1660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":1660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":1660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":8236,"parent_record_offset":8200,"ancestor_record_offset":1660,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":1660,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":1660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":412,"record_offset":8276,"parent_record_offset":8236,"ancestor_record_offset":1660,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":412,"record_offset":6000,"parent_record_offset":5964,"ancestor_record_offset":1660,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":412,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":1660,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":10256,"parent_record_offset":10216,"ancestor_record_offset":1660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6232,"parent_record_offset":6196,"ancestor_record_offset":1660,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":1660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":1660,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6344,"parent_record_offset":6196,"ancestor_record_offset":1660,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":10216,"parent_record_offset":10180,"ancestor_record_offset":1660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6372,"parent_record_offset":6196,"ancestor_record_offset":1660,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":8104,"parent_record_offset":8064,"ancestor_record_offset":1660,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":8136,"parent_record_offset":8104,"ancestor_record_offset":1660,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":6512,"parent_record_offset":6488,"ancestor_record_offset":1660,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":1660,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":412,"record_offset":4660,"parent_record_offset":4432,"ancestor_record_offset":1660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":412,"record_offset":9756,"parent_record_offset":9732,"ancestor_record_offset":1660,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":412,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":1660,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":412,"record_offset":8928,"parent_record_offset":8816,"ancestor_record_offset":1660,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":412,"record_offset":8520,"parent_record_offset":8064,"ancestor_record_offset":1660,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":412,"record_offset":8028,"parent_record_offset":7992,"ancestor_record_offset":1660,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":412,"record_offset":8064,"parent_record_offset":8028,"ancestor_record_offset":1660,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":412,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":1660,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":412,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":1660,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":412,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":1660,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":412,"record_offset":10132,"parent_record_offset":10048,"ancestor_record_offset":1660,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":412,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":1660,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":412,"record_offset":4408,"parent_record_offset":4368,"ancestor_record_offset":1660,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":412,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":1660,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":412,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":1660,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":412,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":1660,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":412,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":1660,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":412,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":1660,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":1660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":1660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3724,"parent_record_offset":3696,"ancestor_record_offset":1660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3644,"parent_record_offset":3616,"ancestor_record_offset":1660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":1660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3840,"parent_record_offset":3668,"ancestor_record_offset":1660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":412,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":1660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":412,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":1660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":412,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":1660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":412,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":1660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":412,"record_offset":7584,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":412,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":1660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":412,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":1660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":412,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":1660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":412,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":1660,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":412,"record_offset":8716,"parent_record_offset":8616,"ancestor_record_offset":1660,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":412,"record_offset":8680,"parent_record_offset":8652,"ancestor_record_offset":1660,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":412,"record_offset":8652,"parent_record_offset":8616,"ancestor_record_offset":1660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":412,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":1660,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":412,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":1660,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":412,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":1660,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":412,"record_offset":8784,"parent_record_offset":8716,"ancestor_record_offset":1660,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":412,"record_offset":6600,"parent_record_offset":6572,"ancestor_record_offset":1660,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":412,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":1660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":412,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":1660,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":412,"record_offset":4616,"parent_record_offset":4588,"ancestor_record_offset":1660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":412,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":1660,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":412,"record_offset":4476,"parent_record_offset":4432,"ancestor_record_offset":1660,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":412,"record_offset":5648,"parent_record_offset":5616,"ancestor_record_offset":1660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":412,"record_offset":2084,"parent_record_offset":2052,"ancestor_record_offset":1660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":412,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":1660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":412,"record_offset":7288,"parent_record_offset":7260,"ancestor_record_offset":1660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":412,"record_offset":3512,"parent_record_offset":3468,"ancestor_record_offset":1660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":412,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":1660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":412,"record_offset":5172,"parent_record_offset":5092,"ancestor_record_offset":1660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":1660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":412,"record_offset":4300,"parent_record_offset":3468,"ancestor_record_offset":1660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":3904,"parent_record_offset":3616,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":1660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":412,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":1660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":5320,"parent_record_offset":5296,"ancestor_record_offset":1660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":6032,"parent_record_offset":6000,"ancestor_record_offset":1660,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":1660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":6728,"parent_record_offset":6660,"ancestor_record_offset":1660,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":1660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":412,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":1660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":1660,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":9680,"parent_record_offset":9652,"ancestor_record_offset":1660,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":412,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":4532,"parent_record_offset":4432,"ancestor_record_offset":1660,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":412,"record_offset":4884,"parent_record_offset":4432,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":4588,"parent_record_offset":4560,"ancestor_record_offset":1660,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":412,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":1660,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":412,"record_offset":4908,"parent_record_offset":4884,"ancestor_record_offset":1660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":1660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":1660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":1660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":5248,"parent_record_offset":5196,"ancestor_record_offset":1660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":2700,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":412,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":1660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":412,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":1660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":412,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":1660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":412,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":1660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":412,"record_offset":2520,"parent_record_offset":2484,"ancestor_record_offset":1660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":412,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":412,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":412,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":412,"record_offset":3024,"parent_record_offset":2484,"ancestor_record_offset":1660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":2024,"parent_record_offset":1992,"ancestor_record_offset":1660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":1660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":2184,"parent_record_offset":2152,"ancestor_record_offset":1660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":2280,"parent_record_offset":2184,"ancestor_record_offset":1660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":1660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":1968,"parent_record_offset":1936,"ancestor_record_offset":1660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":412,"record_offset":2304,"parent_record_offset":2280,"ancestor_record_offset":1660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":412,"record_offset":2584,"parent_record_offset":2552,"ancestor_record_offset":1660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":412,"record_offset":2868,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":412,"record_offset":2840,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":412,"record_offset":2644,"parent_record_offset":2584,"ancestor_record_offset":1660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":6060,"parent_record_offset":6000,"ancestor_record_offset":1660,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":412,"record_offset":6092,"parent_record_offset":5964,"ancestor_record_offset":1660,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":412,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":1660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":412,"record_offset":7228,"parent_record_offset":7200,"ancestor_record_offset":1660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":412,"record_offset":7260,"parent_record_offset":7228,"ancestor_record_offset":1660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":412,"record_offset":7200,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":412,"record_offset":7060,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":412,"record_offset":7612,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":412,"record_offset":7144,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":412,"record_offset":7032,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":412,"record_offset":7492,"parent_record_offset":7468,"ancestor_record_offset":1660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":412,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":1660,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":412,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":1660,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":412,"record_offset":7468,"parent_record_offset":7360,"ancestor_record_offset":1660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":412,"record_offset":6868,"parent_record_offset":6836,"ancestor_record_offset":1660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":1660,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":1660,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":7704,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":1660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":6896,"parent_record_offset":6836,"ancestor_record_offset":1660,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":6948,"parent_record_offset":6920,"ancestor_record_offset":1660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":7004,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":412,"record_offset":7360,"parent_record_offset":6948,"ancestor_record_offset":1660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":3396,"parent_record_offset":3360,"ancestor_record_offset":1660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":10480,"parent_record_offset":10448,"ancestor_record_offset":1660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":10980,"parent_record_offset":10720,"ancestor_record_offset":1660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":412,"record_offset":3540,"parent_record_offset":3512,"ancestor_record_offset":1660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":3616,"parent_record_offset":3468,"ancestor_record_offset":1660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":412,"record_offset":3876,"parent_record_offset":3616,"ancestor_record_offset":1660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":412,"record_offset":3568,"parent_record_offset":3540,"ancestor_record_offset":1660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":412,"record_offset":3468,"parent_record_offset":3432,"ancestor_record_offset":1660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":412,"record_offset":3432,"parent_record_offset":3396,"ancestor_record_offset":1660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/maintain_stone_column_colliders.rs
//     evidence={"module":412,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":1660,"inlinee":24772,"source_line":62,"archive":"_pdb/B/modules/00412.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1145A
// 首条位置: {"module":1415,"inlinee":70746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1415,"record_offset":4896,"parent_record_offset":4736,"ancestor_record_offset":4736,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1415,"record_offset":4868,"parent_record_offset":4736,"ancestor_record_offset":4736,"inlinee":10295,"source_line":74,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1415,"record_offset":4784,"parent_record_offset":4736,"ancestor_record_offset":4736,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1415,"record_offset":4840,"parent_record_offset":4736,"ancestor_record_offset":4736,"inlinee":10293,"source_line":89,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1415,"record_offset":4812,"parent_record_offset":4736,"ancestor_record_offset":4736,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1145B
// 首条位置: {"module":1415,"inlinee":70747}
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
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60C4
// 首条位置: {"module":412,"inlinee":24772}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders
// 信息 ID: ea87413d9749c9ca29253228
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 598d3ba40ebfe6332e15a774: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> > @ 0x33b324 -> 0x78a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::wall_stone_columns::StoneColumnsChanged>)> > > @ 0x432cbf -> 0x78a6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$0
// 信息 ID: 8601c0949ee78b7a8734372a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1415, inline_record=4736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$1
// 信息 ID: 7190751352b9cae618ac45b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1>; module=1415, inline_record=5724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure$2
// 信息 ID: 5ae79de933d5f1a60886a4e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders; module=412, inline_record=8744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
