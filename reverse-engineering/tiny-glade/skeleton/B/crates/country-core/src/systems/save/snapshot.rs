// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/save/snapshot.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b91970eac226f4ef2917c460da0279e9efa179d4c4290559ad97bd075aa351ef
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapshot.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 554, 1585, 1610, 1611, 1624, 1698, 1722
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use flate2; // namespace candidate
//   package identities: [{"name":"flate2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.24"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use itoa; // namespace candidate
//   package identities: [{"name":"itoa","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.3"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use motomoto; // namespace candidate
//   package identities: [{"name":"motomoto","source":"local_or_path","version":"0.1.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use ryu; // namespace candidate
//   package identities: [{"name":"ryu","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.11"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot_save_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1722,"record_offset":6616,"rva":24773680,"va":5393482800}
// 编译实例 RVAs: 0x17a0430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004f570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24453740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e894dd8afe","size":5,"soft_fail":false,"target_va":5369034096,"va":5393487831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecac0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18565977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8a7b4e4fe","size":5,"soft_fail":false,"target_va":5374921408,"va":5393487380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18403208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e87830e7fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393484691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18405553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e84f27e7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393487052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f320
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15869212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8e4da0dff","size":5,"soft_fail":false,"target_va":5377618720,"va":5393487927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15867405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8f3e10dff","size":5,"soft_fail":false,"target_va":5377619792,"va":5393487192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408825a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15855638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8ea0f0eff","size":5,"soft_fail":false,"target_va":5377631648,"va":5393487281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140890240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15800041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e817e90eff","size":5,"soft_fail":false,"target_va":5377688128,"va":5393488164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b734a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12769693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e863263dff","size":5,"soft_fail":false,"target_va":5380715680,"va":5393485368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fbcd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4869511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e879b2b5ff","size":5,"soft_fail":false,"target_va":5388614864,"va":5393484370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141305170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4830857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e87749b6ff","size":5,"soft_fail":false,"target_va":5388652912,"va":5393483764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141306570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4827159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8e957b6ff","size":5,"soft_fail":false,"target_va":5388658032,"va":5393485186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141308b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4816177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8cf82b6ff","size":5,"soft_fail":false,"target_va":5388667712,"va":5393483884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141309840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4812965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e85b8fb6ff","size":5,"soft_fail":false,"target_va":5388671040,"va":5393484000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141477c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3313899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8156fcdff","size":5,"soft_fail":false,"target_va":5390171184,"va":5393485078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157d200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2242353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8cfc8ddff","size":5,"soft_fail":false,"target_va":5391241728,"va":5393484076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18186564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e844811501","size":5,"soft_fail":false,"target_va":5411669968,"va":5393483399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18361658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e83a2d1801","size":5,"soft_fail":false,"target_va":5411847168,"va":5393485505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18054338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8c27c1301","size":5,"soft_fail":false,"target_va":5411540992,"va":5393486649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18054326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8b67c1301","size":5,"soft_fail":false,"target_va":5411540963,"va":5393486632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18055002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e85a7f1301","size":5,"soft_fail":false,"target_va":5411541584,"va":5393486577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18055506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e852811301","size":5,"soft_fail":false,"target_va":5411542064,"va":5393486553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8acf8ffff","size":5,"soft_fail":false,"target_va":5393481024,"va":5393482895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18060376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e858941301","size":5,"soft_fail":false,"target_va":5411543360,"va":5393482979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsSnapshot::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15333446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8ba0716ff","size":5,"soft_fail":false,"target_va":5378150688,"va":5393484129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::Fallibility::alloc_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10648591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8f1835dff","size":5,"soft_fail":false,"target_va":5382837200,"va":5393485786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::Fallibility::capacity_overflow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10648600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8e8835dff","size":5,"soft_fail":false,"target_va":5382837120,"va":5393485715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12097945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8676647ff","size":5,"soft_fail":false,"target_va":5381385536,"va":5393483476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12092724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8cc7a47ff","size":5,"soft_fail":false,"target_va":5381390640,"va":5393483359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12099993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8675e47ff","size":5,"soft_fail":false,"target_va":5381386480,"va":5393486468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12098167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8896547ff","size":5,"soft_fail":false,"target_va":5381384704,"va":5393482866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4661634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e87edeb8ff","size":5,"soft_fail":false,"target_va":5388825296,"va":5393486925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18050400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393482800,"raw_hex":"e8606d1301","size":5,"soft_fail":false,"target_va":5411537104,"va":5393486699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1722,"record_offset":15276,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":13760,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":9292,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1722,"record_offset":9324,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":9844,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":10932,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":28998,"source_line":3362,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":9780,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":15468,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::GateColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1722,"record_offset":13120,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":33128,"source_line":153,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::HalftimberColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1722,"record_offset":13316,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":77370,"source_line":222,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorIds::iter_nondefault
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":1722,"record_offset":12924,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":77369,"source_line":85,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_removed_supports::impl$0::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_removed_supports.rs
//     evidence={"module":1722,"record_offset":14200,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":77374,"source_line":8,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::StairsState::take_snapshot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":11052,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":28286,"source_line":188,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::impl$14::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1722,"record_offset":13512,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":77372,"source_line":43,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1722,"record_offset":13620,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":30724,"source_line":330,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::height_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1722,"record_offset":14172,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":30725,"source_line":334,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorage::take_snapshot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1722,"record_offset":11396,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":31130,"source_line":1059,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::all
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1722,"record_offset":9900,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":26529,"source_line":103,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster_defs::AllWorldRasterRead::read
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1722,"record_offset":12112,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":26231,"source_line":686,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":8284,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":6860,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":7308,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1722,"record_offset":10460,"parent_record_offset":6616,"ancestor_record_offset":6616,"inlinee":24852,"source_line":210,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":6616,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":12712,"parent_record_offset":12688,"ancestor_record_offset":6616,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":6616,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":15312,"parent_record_offset":15276,"ancestor_record_offset":6616,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":12664,"parent_record_offset":12636,"ancestor_record_offset":6616,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":11316,"parent_record_offset":11224,"ancestor_record_offset":6616,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":9584,"parent_record_offset":9560,"ancestor_record_offset":6616,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":12576,"parent_record_offset":12532,"ancestor_record_offset":6616,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":6616,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":6616,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":6616,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":6616,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":12636,"parent_record_offset":12604,"ancestor_record_offset":6616,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":12604,"parent_record_offset":12576,"ancestor_record_offset":6616,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":6616,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":13844,"parent_record_offset":13816,"ancestor_record_offset":6616,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":6616,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":13788,"parent_record_offset":13760,"ancestor_record_offset":6616,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":6616,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":9536,"parent_record_offset":9504,"ancestor_record_offset":6616,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":12800,"parent_record_offset":12768,"ancestor_record_offset":6616,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":11124,"parent_record_offset":11084,"ancestor_record_offset":6616,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":12488,"parent_record_offset":12416,"ancestor_record_offset":6616,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":12532,"parent_record_offset":12488,"ancestor_record_offset":6616,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":9868,"parent_record_offset":9844,"ancestor_record_offset":6616,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":9504,"parent_record_offset":9472,"ancestor_record_offset":6616,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1722,"record_offset":12768,"parent_record_offset":12416,"ancestor_record_offset":6616,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1722,"record_offset":12380,"parent_record_offset":12344,"ancestor_record_offset":6616,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1722,"record_offset":12416,"parent_record_offset":12380,"ancestor_record_offset":6616,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1722,"record_offset":9440,"parent_record_offset":9352,"ancestor_record_offset":6616,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1722,"record_offset":9472,"parent_record_offset":9440,"ancestor_record_offset":6616,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1722,"record_offset":9352,"parent_record_offset":9324,"ancestor_record_offset":6616,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1722,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":6616,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1722,"record_offset":9384,"parent_record_offset":9352,"ancestor_record_offset":6616,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1722,"record_offset":9740,"parent_record_offset":9440,"ancestor_record_offset":6616,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1722,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":6616,"inlinee":4645,"source_line":354,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":6616,"inlinee":4647,"source_line":253,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":6616,"inlinee":4648,"source_line":353,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8672,"parent_record_offset":8644,"ancestor_record_offset":6616,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":6616,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8700,"parent_record_offset":8672,"ancestor_record_offset":6616,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8612,"parent_record_offset":8580,"ancestor_record_offset":6616,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8644,"parent_record_offset":8612,"ancestor_record_offset":6616,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8816,"parent_record_offset":8644,"ancestor_record_offset":6616,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1722,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":6616,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":6616,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":10252,"parent_record_offset":10220,"ancestor_record_offset":6616,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":6616,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":14572,"parent_record_offset":14512,"ancestor_record_offset":6616,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1722,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":6616,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::Copied::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1722,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":6616,"inlinee":29000,"source_line":24,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1722,"record_offset":9808,"parent_record_offset":9780,"ancestor_record_offset":6616,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1722,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":6616,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":11992,"parent_record_offset":11420,"ancestor_record_offset":6616,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":12052,"parent_record_offset":11396,"ancestor_record_offset":6616,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":12832,"parent_record_offset":12800,"ancestor_record_offset":6616,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":12864,"parent_record_offset":12832,"ancestor_record_offset":6616,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":6616,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":6616,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":14604,"parent_record_offset":14480,"ancestor_record_offset":6616,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":14512,"parent_record_offset":14480,"ancestor_record_offset":6616,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":14544,"parent_record_offset":14512,"ancestor_record_offset":6616,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":11284,"parent_record_offset":11252,"ancestor_record_offset":6616,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_neg
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1722,"record_offset":15024,"parent_record_offset":15000,"ancestor_record_offset":6616,"inlinee":6185,"source_line":2107,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1722,"record_offset":15056,"parent_record_offset":15024,"ancestor_record_offset":6616,"inlinee":6186,"source_line":1931,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1722,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":6616,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1722,"record_offset":8448,"parent_record_offset":8408,"ancestor_record_offset":6616,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":6616,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":11864,"parent_record_offset":11420,"ancestor_record_offset":6616,"inlinee":13703,"source_line":1350,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":7256,"parent_record_offset":7228,"ancestor_record_offset":6616,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":8548,"parent_record_offset":8408,"ancestor_record_offset":6616,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":14100,"parent_record_offset":13844,"ancestor_record_offset":6616,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":8880,"parent_record_offset":8580,"ancestor_record_offset":6616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":6616,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":15000,"parent_record_offset":14976,"ancestor_record_offset":6616,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":7720,"parent_record_offset":7696,"ancestor_record_offset":6616,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6616,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":7904,"parent_record_offset":7876,"ancestor_record_offset":6616,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":14764,"parent_record_offset":14736,"ancestor_record_offset":6616,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":6616,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":6616,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1722,"record_offset":9676,"parent_record_offset":9504,"ancestor_record_offset":6616,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":6616,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":6616,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":7976,"parent_record_offset":7948,"ancestor_record_offset":6616,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":7624,"parent_record_offset":7508,"ancestor_record_offset":6616,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1722,"record_offset":6976,"parent_record_offset":6948,"ancestor_record_offset":6616,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1722,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":6616,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1722,"record_offset":7476,"parent_record_offset":7444,"ancestor_record_offset":6616,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1722,"record_offset":7444,"parent_record_offset":7408,"ancestor_record_offset":6616,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1722,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":6616,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1722,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":6616,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1722,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":6616,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1722,"record_offset":7948,"parent_record_offset":7408,"ancestor_record_offset":6616,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":6948,"parent_record_offset":6916,"ancestor_record_offset":6616,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":6916,"parent_record_offset":6892,"ancestor_record_offset":6616,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":7108,"parent_record_offset":7076,"ancestor_record_offset":6616,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":7204,"parent_record_offset":7108,"ancestor_record_offset":6616,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":6616,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":6892,"parent_record_offset":6860,"ancestor_record_offset":6616,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1722,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":6616,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1722,"record_offset":7508,"parent_record_offset":7476,"ancestor_record_offset":6616,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1722,"record_offset":7792,"parent_record_offset":7508,"ancestor_record_offset":6616,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1722,"record_offset":7764,"parent_record_offset":7508,"ancestor_record_offset":6616,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1722,"record_offset":7568,"parent_record_offset":7508,"ancestor_record_offset":6616,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::StairsState::take_snapshot_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":11084,"parent_record_offset":11052,"ancestor_record_offset":6616,"inlinee":28285,"source_line":192,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1722,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":6616,"inlinee":31061,"source_line":742,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::WorldRaster::last_staged
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1722,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":6616,"inlinee":26191,"source_line":192,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":10164,"parent_record_offset":10104,"ancestor_record_offset":6616,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":10220,"parent_record_offset":10164,"ancestor_record_offset":6616,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":6616,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":10516,"parent_record_offset":10488,"ancestor_record_offset":6616,"inlinee":10779,"source_line":650,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":13580,"parent_record_offset":13556,"ancestor_record_offset":6616,"inlinee":11988,"source_line":191,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":10104,"parent_record_offset":10012,"ancestor_record_offset":6616,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::data_start
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14976,"parent_record_offset":14844,"ancestor_record_offset":6616,"inlinee":6183,"source_line":687,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":6616,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14400,"parent_record_offset":14300,"ancestor_record_offset":6616,"inlinee":6140,"source_line":620,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":10076,"parent_record_offset":10012,"ancestor_record_offset":6616,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14948,"parent_record_offset":14844,"ancestor_record_offset":6616,"inlinee":6126,"source_line":2475,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":10048,"parent_record_offset":10012,"ancestor_record_offset":6616,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14344,"parent_record_offset":14300,"ancestor_record_offset":6616,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":10012,"parent_record_offset":9984,"ancestor_record_offset":6616,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::new_uninitialized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14436,"parent_record_offset":14400,"ancestor_record_offset":6616,"inlinee":6141,"source_line":1428,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14880,"parent_record_offset":14844,"ancestor_record_offset":6616,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14480,"parent_record_offset":14436,"ancestor_record_offset":6616,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::alloc::inner::do_alloc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\alloc.rs
//     evidence={"module":1722,"record_offset":14664,"parent_record_offset":14436,"ancestor_record_offset":6616,"inlinee":6144,"source_line":14,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$13::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14300,"parent_record_offset":14272,"ancestor_record_offset":6616,"inlinee":6282,"source_line":3123,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$15::clone_from_spec
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":14844,"parent_record_offset":14300,"ancestor_record_offset":6616,"inlinee":6182,"source_line":3238,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":17688,"parent_record_offset":17664,"ancestor_record_offset":6616,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1722,"record_offset":10488,"parent_record_offset":10460,"ancestor_record_offset":6616,"inlinee":10778,"source_line":290,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$0::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1722,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":6616,"inlinee":15881,"source_line":119,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":8324,"parent_record_offset":8284,"ancestor_record_offset":6616,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":15540,"parent_record_offset":15504,"ancestor_record_offset":6616,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":16092,"parent_record_offset":15820,"ancestor_record_offset":6616,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1722,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":6616,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":8580,"parent_record_offset":8408,"ancestor_record_offset":6616,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1722,"record_offset":8852,"parent_record_offset":8580,"ancestor_record_offset":6616,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":9928,"parent_record_offset":9900,"ancestor_record_offset":6616,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":13556,"parent_record_offset":13512,"ancestor_record_offset":6616,"inlinee":15924,"source_line":1353,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$3::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1722,"record_offset":14224,"parent_record_offset":14200,"ancestor_record_offset":6616,"inlinee":15879,"source_line":1013,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1722,"record_offset":8504,"parent_record_offset":8476,"ancestor_record_offset":6616,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1722,"record_offset":8408,"parent_record_offset":8364,"ancestor_record_offset":6616,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1722,"record_offset":8364,"parent_record_offset":8324,"ancestor_record_offset":6616,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B8C
// 首条位置: {"module":554,"inlinee":31628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1585,"record_offset":18008,"parent_record_offset":17960,"ancestor_record_offset":17960,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":18576,"parent_record_offset":18552,"ancestor_record_offset":17960,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":18600,"parent_record_offset":18576,"ancestor_record_offset":17960,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1585,"record_offset":18552,"parent_record_offset":18372,"ancestor_record_offset":17960,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":18304,"parent_record_offset":18272,"ancestor_record_offset":17960,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":18372,"parent_record_offset":18336,"ancestor_record_offset":17960,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":18336,"parent_record_offset":18304,"ancestor_record_offset":17960,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1585,"record_offset":18412,"parent_record_offset":18372,"ancestor_record_offset":17960,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1585,"record_offset":18232,"parent_record_offset":18200,"ancestor_record_offset":17960,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1585,"record_offset":18200,"parent_record_offset":18168,"ancestor_record_offset":17960,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1585,"record_offset":18136,"parent_record_offset":18104,"ancestor_record_offset":17960,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1585,"record_offset":18168,"parent_record_offset":18136,"ancestor_record_offset":17960,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1585,"record_offset":18104,"parent_record_offset":18072,"ancestor_record_offset":17960,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1585,"record_offset":18072,"parent_record_offset":18040,"ancestor_record_offset":17960,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1585,"record_offset":18040,"parent_record_offset":18008,"ancestor_record_offset":17960,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":18272,"parent_record_offset":18232,"ancestor_record_offset":17960,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1585,"record_offset":18436,"parent_record_offset":18412,"ancestor_record_offset":17960,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1585,"record_offset":18460,"parent_record_offset":18436,"ancestor_record_offset":17960,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":18676,"parent_record_offset":18652,"ancestor_record_offset":17960,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1585,"record_offset":18508,"parent_record_offset":18484,"ancestor_record_offset":17960,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1585,"record_offset":18484,"parent_record_offset":18460,"ancestor_record_offset":17960,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1585,"record_offset":18652,"parent_record_offset":18232,"ancestor_record_offset":17960,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B8D
// 首条位置: {"module":554,"inlinee":31629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":554,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":600,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":600,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":600,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":600,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":1160,"parent_record_offset":984,"ancestor_record_offset":600,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":2084,"parent_record_offset":1996,"ancestor_record_offset":600,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":924,"parent_record_offset":896,"ancestor_record_offset":600,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":2024,"parent_record_offset":1996,"ancestor_record_offset":600,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1996,"parent_record_offset":1972,"ancestor_record_offset":600,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":984,"parent_record_offset":952,"ancestor_record_offset":600,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":952,"parent_record_offset":924,"ancestor_record_offset":600,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":600,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":600,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":860,"parent_record_offset":832,"ancestor_record_offset":600,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":832,"parent_record_offset":768,"ancestor_record_offset":600,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":804,"parent_record_offset":768,"ancestor_record_offset":600,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":896,"parent_record_offset":860,"ancestor_record_offset":600,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":740,"parent_record_offset":692,"ancestor_record_offset":600,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":768,"parent_record_offset":740,"ancestor_record_offset":600,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":600,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":600,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":600,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":600,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":600,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":2052,"parent_record_offset":2024,"ancestor_record_offset":600,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":1260,"parent_record_offset":860,"ancestor_record_offset":600,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":1924,"parent_record_offset":692,"ancestor_record_offset":600,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":554,"record_offset":2160,"parent_record_offset":628,"ancestor_record_offset":600,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":554,"record_offset":692,"parent_record_offset":628,"ancestor_record_offset":600,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AC4
// 首条位置: {"module":1585,"inlinee":72388}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":71172,"parent_record_offset":71132,"ancestor_record_offset":71132,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":71744,"parent_record_offset":71720,"ancestor_record_offset":71132,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":71768,"parent_record_offset":71744,"ancestor_record_offset":71132,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":71720,"parent_record_offset":71488,"ancestor_record_offset":71132,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":71396,"parent_record_offset":71352,"ancestor_record_offset":71132,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":71488,"parent_record_offset":71440,"ancestor_record_offset":71132,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":71440,"parent_record_offset":71396,"ancestor_record_offset":71132,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":71544,"parent_record_offset":71488,"ancestor_record_offset":71132,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":71304,"parent_record_offset":71260,"ancestor_record_offset":71132,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":71260,"parent_record_offset":71216,"ancestor_record_offset":71132,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":71216,"parent_record_offset":71172,"ancestor_record_offset":71132,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":71352,"parent_record_offset":71304,"ancestor_record_offset":71132,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1610,"record_offset":71580,"parent_record_offset":71544,"ancestor_record_offset":71132,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1610,"record_offset":71616,"parent_record_offset":71580,"ancestor_record_offset":71132,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":71844,"parent_record_offset":71820,"ancestor_record_offset":71132,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":71672,"parent_record_offset":71644,"ancestor_record_offset":71132,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1610,"record_offset":71644,"parent_record_offset":71616,"ancestor_record_offset":71132,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":71820,"parent_record_offset":71304,"ancestor_record_offset":71132,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::capture_game_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1722,"record_offset":19228,"rva":24780032,"va":5393489152}
// 编译实例 RVAs: 0x17a1d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14004f570
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24465920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e800ae8afe","size":5,"soft_fail":false,"target_va":5369034096,"va":5393500011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23440105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e817559afe","size":5,"soft_fail":false,"target_va":5370060624,"va":5393500724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7890
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18603000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e80824e4fe","size":5,"soft_fail":false,"target_va":5374900368,"va":5393503363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=8
//     target_aliases=92 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18594586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8e644e4fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393495317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=13
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18600895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8412ce4fe","size":5,"soft_fail":false,"target_va":5374904480,"va":5393505370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ecac0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18587960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8c85ee4fe","size":5,"soft_fail":false,"target_va":5374921408,"va":5393509363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18408126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8421de7fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393489609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=93
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18408596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e86c1be7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393490095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16019353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e867900bff","size":5,"soft_fail":false,"target_va":5377481360,"va":5393500708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140876c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15924315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8a5030dff","size":5,"soft_fail":false,"target_va":5377584128,"va":5393508438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087abf0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15900072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e858620dff","size":5,"soft_fail":false,"target_va":5377600496,"va":5393500563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15882056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8b8a80dff","size":5,"soft_fail":false,"target_va":5377618720,"va":5393500771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15881240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8e8ab0dff","size":5,"soft_fail":false,"target_va":5377619792,"va":5393501027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140881890
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15867691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8d5e00dff","size":5,"soft_fail":false,"target_va":5377628304,"va":5393495990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408825a0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15869338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e866da0dff","size":5,"soft_fail":false,"target_va":5377631648,"va":5393500981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=15
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15831842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8de6c0eff","size":5,"soft_fail":false,"target_va":5377658224,"va":5393490061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=16
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15765890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e87e6e0fff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393490109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e6b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15449919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8c14014ff","size":5,"soft_fail":false,"target_va":5378042736,"va":5393492650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14996004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8dc2d1bff","size":5,"soft_fail":false,"target_va":5378507024,"va":5393503023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14995722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8f62e1bff","size":5,"soft_fail":false,"target_va":5378507344,"va":5393503061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14995008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8c0311bff","size":5,"soft_fail":false,"target_va":5378508096,"va":5393503099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14993198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8d2381bff","size":5,"soft_fail":false,"target_va":5378509792,"va":5393502985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958d60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14992966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8ba391bff","size":5,"soft_fail":false,"target_va":5378510176,"va":5393503137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a146f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14217283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8bd0f27ff","size":5,"soft_fail":false,"target_va":5379278576,"va":5393495854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a15220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14216377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8471327ff","size":5,"soft_fail":false,"target_va":5379281440,"va":5393497812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140af56a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13295842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e81e1f35ff","size":5,"soft_fail":false,"target_va":5380200096,"va":5393495933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b30f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13049017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e847e338ff","size":5,"soft_fail":false,"target_va":5380443952,"va":5393492964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3bad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13006339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8fd8939ff","size":5,"soft_fail":false,"target_va":5380487888,"va":5393494222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141338990
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4637360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8503db9ff","size":5,"soft_fail":false,"target_va":5388863888,"va":5393501243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141338d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4636244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8ac41b9ff","size":5,"soft_fail":false,"target_va":5388864896,"va":5393501135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141339130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4635246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e89245b9ff","size":5,"soft_fail":false,"target_va":5388865840,"va":5393501081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413392a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4634986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e89646b9ff","size":5,"soft_fail":false,"target_va":5388866208,"va":5393501189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414397d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3582968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e80854c9ff","size":5,"soft_fail":false,"target_va":5389916112,"va":5393499075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143ba30
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3565072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8f099c9ff","size":5,"soft_fail":false,"target_va":5389924912,"va":5393489979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14154fcf0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2440369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e84fc3daff","size":5,"soft_fail":false,"target_va":5391056112,"va":5393496476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141552690
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2427759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e891f4daff","size":5,"soft_fail":false,"target_va":5391066768,"va":5393494522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141552820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2428383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e821f2daff","size":5,"soft_fail":false,"target_va":5391067168,"va":5393495546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141560e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2364677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8fbeadbff","size":5,"soft_fail":false,"target_va":5391126160,"va":5393490832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157d810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2250625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e87fa8ddff","size":5,"soft_fail":false,"target_va":5391243280,"va":5393493900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415802e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2240746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e816cfddff","size":5,"soft_fail":false,"target_va":5391254240,"va":5393494981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141581f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2233281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e83fecddff","size":5,"soft_fail":false,"target_va":5391261472,"va":5393494748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158b1c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2200091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8e56ddeff","size":5,"soft_fail":false,"target_va":5391299008,"va":5393499094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158f050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2178573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8f3c1deff","size":5,"soft_fail":false,"target_va":5391315024,"va":5393493592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158f8b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2174413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e833d2deff","size":5,"soft_fail":false,"target_va":5391317168,"va":5393491576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158faf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2172775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e899d8deff","size":5,"soft_fail":false,"target_va":5391317744,"va":5393490514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141590160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2173886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e842d4deff","size":5,"soft_fail":false,"target_va":5391319392,"va":5393493273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415905e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2170782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e862e0deff","size":5,"soft_fail":false,"target_va":5391320544,"va":5393491321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141590a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2168562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e80ee9deff","size":5,"soft_fail":false,"target_va":5391321648,"va":5393490205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8e80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17846987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8cb521001","size":5,"soft_fail":false,"target_va":5411344000,"va":5393497008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9590
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17851079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8c7621001","size":5,"soft_fail":false,"target_va":5411345808,"va":5393494724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17847135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e85f531001","size":5,"soft_fail":false,"target_va":5411346512,"va":5393499372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18352029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e89d071801","size":5,"soft_fail":false,"target_va":5411847168,"va":5393495134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18352524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e88c091801","size":5,"soft_fail":false,"target_va":5411847200,"va":5393494671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t17850784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8a0611001","size":5,"soft_fail":false,"target_va":5411347040,"va":5393496251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t18039278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8ee411301","size":5,"soft_fail":false,"target_va":5411540963,"va":5393501680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18038856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e848401301","size":5,"soft_fail":false,"target_va":5411541776,"va":5393502915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t18041045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8d5481301","size":5,"soft_fail":false,"target_va":5411542880,"va":5393501830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] flate2::mem::Compress::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12467054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e892c441ff","size":5,"soft_fail":false,"target_va":5381028672,"va":5393495721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] motomoto::tag_from_str_trunc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9295636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e814d78d00","size":5,"soft_fail":false,"target_va":5402791168,"va":5393495527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ryu::pretty::format64
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12678663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8f9893eff","size":5,"soft_fail":false,"target_va":5380819888,"va":5393498546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::error::Error::io
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17982661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8c5641201","size":5,"soft_fail":false,"target_va":5411477664,"va":5393494998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10802255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393489152,"raw_hex":"e8b12b5bff","size":5,"soft_fail":false,"target_va":5382694656,"va":5393496906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5393489152,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5393499659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1722,"record_offset":65820,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":19456,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":52212,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":52268,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":53604,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1722,"record_offset":52460,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":9455,"source_line":1367,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1722,"record_offset":20896,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":27640,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":21504,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":21448,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":43912,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":52184,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":20332,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":24453,"source_line":2972,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by_key
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":19732,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":11461,"source_line":3069,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::DecoratorStorageSnapshot::sort
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1722,"record_offset":20012,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":31134,"source_line":1240,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::write::DeflateEncoder::finish
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/deflate/write.rs
//     evidence={"module":1722,"record_offset":53720,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":8695,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::write::DeflateEncoder::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/deflate/write.rs
//     evidence={"module":1722,"record_offset":53268,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":8694,"source_line":40,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] motomoto::Writer::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1722,"record_offset":19704,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":73989,"source_line":74,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":21476,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":13849,"source_line":125,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":20868,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":13848,"source_line":38,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":43944,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1722,"record_offset":22928,"parent_record_offset":19228,"ancestor_record_offset":19228,"inlinee":8892,"source_line":940,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":19624,"parent_record_offset":19600,"ancestor_record_offset":19228,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":19648,"parent_record_offset":19624,"ancestor_record_offset":19228,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=107
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":19228,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":19600,"parent_record_offset":19572,"ancestor_record_offset":19228,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=107
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":21712,"parent_record_offset":21656,"ancestor_record_offset":19228,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1722,"record_offset":60232,"parent_record_offset":60204,"ancestor_record_offset":19228,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":50092,"parent_record_offset":50068,"ancestor_record_offset":19228,"inlinee":11039,"source_line":2454,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":19228,"inlinee":14401,"source_line":2577,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":44524,"parent_record_offset":44500,"ancestor_record_offset":19228,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21864,"parent_record_offset":21840,"ancestor_record_offset":19228,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":44640,"parent_record_offset":44612,"ancestor_record_offset":19228,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":19228,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":44548,"parent_record_offset":44524,"ancestor_record_offset":19228,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=99
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21684,"parent_record_offset":21656,"ancestor_record_offset":19228,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21656,"parent_record_offset":21632,"ancestor_record_offset":19228,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":44700,"parent_record_offset":44668,"ancestor_record_offset":19228,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":19228,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":19228,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":44668,"parent_record_offset":44640,"ancestor_record_offset":19228,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":19572,"parent_record_offset":19540,"ancestor_record_offset":19228,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":19540,"parent_record_offset":19512,"ancestor_record_offset":19228,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=100
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":19228,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":21108,"parent_record_offset":21080,"ancestor_record_offset":19228,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":21080,"parent_record_offset":21052,"ancestor_record_offset":19228,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":21024,"parent_record_offset":20980,"ancestor_record_offset":19228,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":19228,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1722,"record_offset":20980,"parent_record_offset":20952,"ancestor_record_offset":19228,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1722,"record_offset":20952,"parent_record_offset":20924,"ancestor_record_offset":19228,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1722,"record_offset":47848,"parent_record_offset":47824,"ancestor_record_offset":19228,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1722,"record_offset":47824,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1722,"record_offset":20924,"parent_record_offset":20896,"ancestor_record_offset":19228,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":21840,"parent_record_offset":21816,"ancestor_record_offset":19228,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":47896,"parent_record_offset":47872,"ancestor_record_offset":19228,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":47872,"parent_record_offset":47848,"ancestor_record_offset":19228,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":44500,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":7415,"source_line":1246,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":28120,"parent_record_offset":28092,"ancestor_record_offset":19228,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":44584,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":44612,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":44908,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":44796,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":8644,"source_line":2876,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":27800,"parent_record_offset":27748,"ancestor_record_offset":19228,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":19484,"parent_record_offset":19456,"ancestor_record_offset":19228,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":27676,"parent_record_offset":27640,"ancestor_record_offset":19228,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":28308,"parent_record_offset":28276,"ancestor_record_offset":19228,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":21816,"parent_record_offset":21788,"ancestor_record_offset":19228,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":55108,"parent_record_offset":55084,"ancestor_record_offset":19228,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":55440,"parent_record_offset":55416,"ancestor_record_offset":19228,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1722,"record_offset":28396,"parent_record_offset":28308,"ancestor_record_offset":19228,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1722,"record_offset":28092,"parent_record_offset":27748,"ancestor_record_offset":19228,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1722,"record_offset":27712,"parent_record_offset":27676,"ancestor_record_offset":19228,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1722,"record_offset":27748,"parent_record_offset":27712,"ancestor_record_offset":19228,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1722,"record_offset":53680,"parent_record_offset":53640,"ancestor_record_offset":19228,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":53640,"parent_record_offset":53604,"ancestor_record_offset":19228,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::io::WriteBytesExt::write_u64
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1722,"record_offset":50420,"parent_record_offset":50376,"ancestor_record_offset":19228,"inlinee":9455,"source_line":1367,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1722,"record_offset":60264,"parent_record_offset":60232,"ancestor_record_offset":19228,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":49916,"parent_record_offset":49888,"ancestor_record_offset":19228,"inlinee":4913,"source_line":1740,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":20708,"parent_record_offset":20684,"ancestor_record_offset":19228,"inlinee":7225,"source_line":1824,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::classify
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":1722,"record_offset":46016,"parent_record_offset":45972,"ancestor_record_offset":19228,"inlinee":6858,"source_line":645,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":44988,"parent_record_offset":44964,"ancestor_record_offset":19228,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":20648,"parent_record_offset":20624,"ancestor_record_offset":19228,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":19808,"parent_record_offset":19768,"ancestor_record_offset":19228,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1722,"record_offset":28176,"parent_record_offset":28148,"ancestor_record_offset":19228,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1722,"record_offset":28244,"parent_record_offset":28204,"ancestor_record_offset":19228,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":28148,"parent_record_offset":28120,"ancestor_record_offset":19228,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":28276,"parent_record_offset":28244,"ancestor_record_offset":19228,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":53868,"parent_record_offset":53832,"ancestor_record_offset":19228,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":60288,"parent_record_offset":60264,"ancestor_record_offset":19228,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":61220,"parent_record_offset":61192,"ancestor_record_offset":19228,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":44472,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=59
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":22060,"parent_record_offset":22032,"ancestor_record_offset":19228,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":44724,"parent_record_offset":44700,"ancestor_record_offset":19228,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1722,"record_offset":20684,"parent_record_offset":20560,"ancestor_record_offset":19228,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1722,"record_offset":46100,"parent_record_offset":46072,"ancestor_record_offset":19228,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":53832,"parent_record_offset":53804,"ancestor_record_offset":19228,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":49888,"parent_record_offset":48984,"ancestor_record_offset":19228,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":21360,"parent_record_offset":21108,"ancestor_record_offset":19228,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":45448,"parent_record_offset":45296,"ancestor_record_offset":19228,"inlinee":8514,"source_line":420,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=234
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":20600,"parent_record_offset":20560,"ancestor_record_offset":19228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":20504,"parent_record_offset":20464,"ancestor_record_offset":19228,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":44964,"parent_record_offset":44940,"ancestor_record_offset":19228,"inlinee":9440,"source_line":1455,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":61252,"parent_record_offset":61160,"ancestor_record_offset":19228,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":60032,"parent_record_offset":60008,"ancestor_record_offset":19228,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":60080,"parent_record_offset":60056,"ancestor_record_offset":19228,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":60056,"parent_record_offset":60032,"ancestor_record_offset":19228,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":28436,"parent_record_offset":28204,"ancestor_record_offset":19228,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":52360,"parent_record_offset":52268,"ancestor_record_offset":19228,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":52304,"parent_record_offset":52268,"ancestor_record_offset":19228,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":52388,"parent_record_offset":52360,"ancestor_record_offset":19228,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":28348,"parent_record_offset":28308,"ancestor_record_offset":19228,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":45072,"parent_record_offset":44228,"ancestor_record_offset":19228,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":53772,"parent_record_offset":53720,"ancestor_record_offset":19228,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":44852,"parent_record_offset":44420,"ancestor_record_offset":19228,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::sort_unstable_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":20048,"parent_record_offset":20012,"ancestor_record_offset":19228,"inlinee":11461,"source_line":3069,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":28204,"parent_record_offset":28176,"ancestor_record_offset":19228,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":1722,"record_offset":20624,"parent_record_offset":20600,"ancestor_record_offset":19228,"inlinee":10771,"source_line":302,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":1722,"record_offset":20560,"parent_record_offset":20464,"ancestor_record_offset":19228,"inlinee":10769,"source_line":542,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":1722,"record_offset":20464,"parent_record_offset":20368,"ancestor_record_offset":19228,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":1722,"record_offset":19768,"parent_record_offset":19732,"ancestor_record_offset":19228,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1722,"record_offset":44876,"parent_record_offset":44852,"ancestor_record_offset":19228,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] flate2::zio::Writer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/zio.rs
//     evidence={"module":1722,"record_offset":53304,"parent_record_offset":53268,"ancestor_record_offset":19228,"inlinee":8676,"source_line":162,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] flate2::zio::Writer::take_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/zio.rs
//     evidence={"module":1722,"record_offset":53804,"parent_record_offset":53720,"ancestor_record_offset":19228,"inlinee":8677,"source_line":199,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61160,"parent_record_offset":61128,"ancestor_record_offset":19228,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61072,"parent_record_offset":61048,"ancestor_record_offset":19228,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61128,"parent_record_offset":61072,"ancestor_record_offset":19228,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61100,"parent_record_offset":61072,"ancestor_record_offset":19228,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61192,"parent_record_offset":61160,"ancestor_record_offset":19228,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":61048,"parent_record_offset":61024,"ancestor_record_offset":19228,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] itoa::Buffer::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1722,"record_offset":45264,"parent_record_offset":45228,"ancestor_record_offset":19228,"inlinee":5442,"source_line":86,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$16::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1722,"record_offset":45296,"parent_record_offset":45264,"ancestor_record_offset":19228,"inlinee":14397,"source_line":125,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] itoa::impl$18::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itoa","version":"1.0.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itoa-1.0.3/src/lib.rs
//     evidence={"module":1722,"record_offset":47128,"parent_record_offset":47096,"ancestor_record_offset":19228,"inlinee":5444,"source_line":125,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::ChunkWriter::seek_back_and_write_length
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1722,"record_offset":50376,"parent_record_offset":50340,"ancestor_record_offset":19228,"inlinee":26904,"source_line":114,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::impl$3::write
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1722,"record_offset":44320,"parent_record_offset":44292,"ancestor_record_offset":19228,"inlinee":32696,"source_line":104,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] motomoto::impl$5::drop
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs
//     evidence={"module":1722,"record_offset":50340,"parent_record_offset":50308,"ancestor_record_offset":19228,"inlinee":26902,"source_line":132,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::Buffer::format_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1722,"record_offset":46164,"parent_record_offset":46132,"ancestor_record_offset":19228,"inlinee":9368,"source_line":72,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] ryu::buffer::impl$7::write_to_ryu_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ryu","version":"1.0.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ryu-1.0.11/src/buffer/mod.rs
//     evidence={"module":1722,"record_offset":46188,"parent_record_offset":46164,"ancestor_record_offset":19228,"inlinee":10885,"source_line":167,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1722,"record_offset":22952,"parent_record_offset":22928,"ancestor_record_offset":19228,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$89::serialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":1722,"record_offset":22976,"parent_record_offset":22952,"ancestor_record_offset":19228,"inlinee":5194,"source_line":200,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::Map::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":48928,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14400,"source_line":226,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$15::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":50068,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14403,"source_line":827,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$18::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":50124,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14405,"source_line":458,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$6::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1722,"record_offset":45132,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14393,"source_line":316,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":49040,"parent_record_offset":48984,"ancestor_record_offset":19228,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":49952,"parent_record_offset":48984,"ancestor_record_offset":19228,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":48140,"parent_record_offset":48108,"ancestor_record_offset":19228,"inlinee":14399,"source_line":1608,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":46132,"parent_record_offset":45972,"ancestor_record_offset":19228,"inlinee":10883,"source_line":1721,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":45228,"parent_record_offset":45196,"ancestor_record_offset":19228,"inlinee":14395,"source_line":1655,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_null
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":44264,"parent_record_offset":44228,"ancestor_record_offset":19228,"inlinee":9365,"source_line":1599,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::write_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":47056,"parent_record_offset":47020,"ancestor_record_offset":19228,"inlinee":5440,"source_line":1699,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":44048,"parent_record_offset":43944,"ancestor_record_offset":19228,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":44072,"parent_record_offset":44048,"ancestor_record_offset":19228,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":48108,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14398,"source_line":77,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_f64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":45972,"parent_record_offset":45132,"ancestor_record_offset":19228,"inlinee":10882,"source_line":193,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_i64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":45196,"parent_record_offset":45132,"ancestor_record_offset":19228,"inlinee":14394,"source_line":113,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":48984,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":48020,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":5182,"source_line":219,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":47020,"parent_record_offset":45132,"ancestor_record_offset":19228,"inlinee":5439,"source_line":157,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_unit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":44228,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":14121,"source_line":235,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":1722,"record_offset":50152,"parent_record_offset":44104,"ancestor_record_offset":19228,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$0::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/ser.rs
//     evidence={"module":1722,"record_offset":44104,"parent_record_offset":43944,"ancestor_record_offset":19228,"inlinee":14440,"source_line":17,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1722,"record_offset":44292,"parent_record_offset":44264,"ancestor_record_offset":19228,"inlinee":8631,"source_line":1754,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$8::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1722,"record_offset":44348,"parent_record_offset":44320,"ancestor_record_offset":19228,"inlinee":8640,"source_line":593,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1722,"record_offset":44420,"parent_record_offset":44376,"ancestor_record_offset":19228,"inlinee":8643,"source_line":443,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1722,"record_offset":44376,"parent_record_offset":44348,"ancestor_record_offset":19228,"inlinee":8642,"source_line":504,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_unchecked
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1722,"record_offset":44940,"parent_record_offset":44376,"ancestor_record_offset":19228,"inlinee":8648,"source_line":486,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1722,"record_offset":60008,"parent_record_offset":59980,"ancestor_record_offset":19228,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1722,"record_offset":59980,"parent_record_offset":59952,"ancestor_record_offset":19228,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::get_param
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120DD
// 首条位置: {"module":1611,"inlinee":73949}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$125::get_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2673044,"parent_record_offset":2673004,"ancestor_record_offset":2673004,"inlinee":5412,"source_line":2106,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":2673572,"parent_record_offset":2673548,"ancestor_record_offset":2673004,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":2673620,"parent_record_offset":2673596,"ancestor_record_offset":2673004,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":2673596,"parent_record_offset":2673572,"ancestor_record_offset":2673004,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":2673548,"parent_record_offset":2673524,"ancestor_record_offset":2673004,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":2673524,"parent_record_offset":2673484,"ancestor_record_offset":2673004,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":2673484,"parent_record_offset":2673460,"ancestor_record_offset":2673004,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::_::impl$0::get_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1611,"record_offset":2683868,"parent_record_offset":2673044,"ancestor_record_offset":2673004,"inlinee":4185,"source_line":61,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1611,"record_offset":2673888,"parent_record_offset":2673864,"ancestor_record_offset":2673004,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1611,"record_offset":2673864,"parent_record_offset":2673832,"ancestor_record_offset":2673004,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1611,"record_offset":2673804,"parent_record_offset":2673768,"ancestor_record_offset":2673004,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":2673740,"parent_record_offset":2673192,"ancestor_record_offset":2673004,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":2673768,"parent_record_offset":2673740,"ancestor_record_offset":2673004,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":2673232,"parent_record_offset":2673192,"ancestor_record_offset":2673004,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":2673832,"parent_record_offset":2673768,"ancestor_record_offset":2673004,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":2673280,"parent_record_offset":2673256,"ancestor_record_offset":2673004,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":2673256,"parent_record_offset":2673232,"ancestor_record_offset":2673004,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":2673460,"parent_record_offset":2673436,"ancestor_record_offset":2673004,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$107::get_param
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2673120,"parent_record_offset":2673084,"ancestor_record_offset":2673004,"inlinee":4315,"source_line":2106,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2673156,"parent_record_offset":2673120,"ancestor_record_offset":2673004,"inlinee":4173,"source_line":1020,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::get_param::closure$0
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2674072,"parent_record_offset":2674048,"ancestor_record_offset":2673004,"inlinee":4208,"source_line":1028,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$97::get_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":2683900,"parent_record_offset":2683868,"ancestor_record_offset":2673004,"inlinee":4186,"source_line":2106,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":1611,"record_offset":2673956,"parent_record_offset":2673932,"ancestor_record_offset":2673004,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1611,"record_offset":2673932,"parent_record_offset":2673192,"ancestor_record_offset":2673004,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":1611,"record_offset":2673192,"parent_record_offset":2673156,"ancestor_record_offset":2673004,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":1611,"record_offset":2674188,"parent_record_offset":2674072,"ancestor_record_offset":2673004,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":1611,"record_offset":2674128,"parent_record_offset":2674104,"ancestor_record_offset":2673004,"inlinee":4202,"source_line":99,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::rt::Argument::new_display
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\rt.rs
//     evidence={"module":1611,"record_offset":2674104,"parent_record_offset":2674072,"ancestor_record_offset":2673004,"inlinee":4201,"source_line":113,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1611,"record_offset":2673372,"parent_record_offset":2673308,"ancestor_record_offset":2673004,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":2673396,"parent_record_offset":2673372,"ancestor_record_offset":2673004,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":2673436,"parent_record_offset":2673256,"ancestor_record_offset":2673004,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":2674048,"parent_record_offset":2673156,"ancestor_record_offset":2673004,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::from_ref
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1611,"record_offset":2674152,"parent_record_offset":2674128,"ancestor_record_offset":2673004,"inlinee":4204,"source_line":268,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1611,"record_offset":2673308,"parent_record_offset":2673280,"ancestor_record_offset":2673004,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1611,"record_offset":2673688,"parent_record_offset":2673664,"ancestor_record_offset":2673004,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1611,"record_offset":2673344,"parent_record_offset":2673308,"ancestor_record_offset":2673004,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1611,"record_offset":2673664,"parent_record_offset":2673484,"ancestor_record_offset":2673004,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1611,"record_offset":2673980,"parent_record_offset":2673956,"ancestor_record_offset":2673004,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1611,"record_offset":2674004,"parent_record_offset":2673980,"ancestor_record_offset":2673004,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster_defs::_::impl$0::get_param
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1611,"record_offset":2673084,"parent_record_offset":2673044,"ancestor_record_offset":2673004,"inlinee":14094,"source_line":695,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::init_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1722,"record_offset":107708,"rva":24818320,"va":5393527440}
// 编译实例 RVAs: 0x17ab290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bevy_ecs..change_detection..ResMut$LT$T$GT$$u20$as$u20$bevy_ecs..system..system_param..SystemParam$GT$::init_state
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t-4426919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393527440,"raw_hex":"e85973bcff","size":5,"soft_fail":false,"target_va":5389100560,"va":5393527474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$125::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1722,"record_offset":107852,"parent_record_offset":107708,"ancestor_record_offset":107708,"inlinee":5407,"source_line":2064,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::_::impl$0::init_state
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1722,"record_offset":107928,"parent_record_offset":107852,"ancestor_record_offset":107708,"inlinee":4109,"source_line":61,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$107::init_state
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1722,"record_offset":107896,"parent_record_offset":107876,"ancestor_record_offset":107708,"inlinee":4310,"source_line":2064,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$97::init_state
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1722,"record_offset":107952,"parent_record_offset":107928,"ancestor_record_offset":107708,"inlinee":4111,"source_line":2064,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster_defs::_::impl$0::init_state
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1722,"record_offset":107876,"parent_record_offset":107852,"ancestor_record_offset":107708,"inlinee":26245,"source_line":695,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120DB
// 首条位置: {"module":1611,"inlinee":73947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::_::impl$0::validate_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1611,"record_offset":1396948,"parent_record_offset":1392480,"ancestor_record_offset":1392480,"inlinee":4227,"source_line":61,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::system_param::impl$12::validate_param
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":1395172,"parent_record_offset":1392480,"ancestor_record_offset":1392480,"inlinee":4225,"source_line":1000,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1611,"record_offset":1399480,"parent_record_offset":1392480,"ancestor_record_offset":1392480,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster_defs::_::impl$0::validate_param
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":1611,"record_offset":1392528,"parent_record_offset":1392480,"ancestor_record_offset":1392480,"inlinee":14088,"source_line":695,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":1399896,"parent_record_offset":1399872,"ancestor_record_offset":1392480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":1399944,"parent_record_offset":1399920,"ancestor_record_offset":1392480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":1399920,"parent_record_offset":1399896,"ancestor_record_offset":1392480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":1399872,"parent_record_offset":1399848,"ancestor_record_offset":1392480,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":1399848,"parent_record_offset":1399816,"ancestor_record_offset":1392480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1611,"record_offset":1399816,"parent_record_offset":1399792,"ancestor_record_offset":1392480,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::_::impl$0::validate_param::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1611,"record_offset":1397436,"parent_record_offset":1397412,"ancestor_record_offset":1392480,"inlinee":4237,"source_line":61,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":1611,"record_offset":1392944,"parent_record_offset":1392920,"ancestor_record_offset":1392480,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":1392920,"parent_record_offset":1392896,"ancestor_record_offset":1392480,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":1611,"record_offset":1392584,"parent_record_offset":1392552,"ancestor_record_offset":1392480,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":1392632,"parent_record_offset":1392608,"ancestor_record_offset":1392480,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":1392608,"parent_record_offset":1392584,"ancestor_record_offset":1392480,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1611,"record_offset":1392804,"parent_record_offset":1392780,"ancestor_record_offset":1392480,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::SystemParamValidationError::invalid
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":1401452,"parent_record_offset":1400860,"ancestor_record_offset":1392480,"inlinee":73948,"source_line":2705,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::SystemParamValidationError::new
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":1401476,"parent_record_offset":1401452,"ancestor_record_offset":1392480,"inlinee":8575,"source_line":2711,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$12::validate_param
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1611,"record_offset":1392552,"parent_record_offset":1392528,"ancestor_record_offset":1392480,"inlinee":4225,"source_line":1000,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1611,"record_offset":1392716,"parent_record_offset":1392656,"ancestor_record_offset":1392480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":1392740,"parent_record_offset":1392716,"ancestor_record_offset":1392480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":1392872,"parent_record_offset":1392552,"ancestor_record_offset":1392480,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1611,"record_offset":1392780,"parent_record_offset":1392608,"ancestor_record_offset":1392480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1611,"record_offset":1397412,"parent_record_offset":1396948,"ancestor_record_offset":1392480,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1611,"record_offset":1392656,"parent_record_offset":1392632,"ancestor_record_offset":1392480,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1611,"record_offset":1392688,"parent_record_offset":1392656,"ancestor_record_offset":1392480,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1611,"record_offset":1392828,"parent_record_offset":1392804,"ancestor_record_offset":1392480,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D5
// 首条位置: {"module":1611,"inlinee":73941}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":461396,"parent_record_offset":461372,"ancestor_record_offset":461372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":461564,"parent_record_offset":461540,"ancestor_record_offset":461372,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":461540,"parent_record_offset":461516,"ancestor_record_offset":461372,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":461516,"parent_record_offset":461492,"ancestor_record_offset":461372,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":461492,"parent_record_offset":461468,"ancestor_record_offset":461372,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":461420,"parent_record_offset":461396,"ancestor_record_offset":461372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D9
// 首条位置: {"module":1611,"inlinee":73945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":464612,"parent_record_offset":464588,"ancestor_record_offset":464588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":464780,"parent_record_offset":464756,"ancestor_record_offset":464588,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":464756,"parent_record_offset":464732,"ancestor_record_offset":464588,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":464732,"parent_record_offset":464708,"ancestor_record_offset":464588,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":464708,"parent_record_offset":464684,"ancestor_record_offset":464588,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":464636,"parent_record_offset":464612,"ancestor_record_offset":464588,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D6
// 首条位置: {"module":1611,"inlinee":73942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":462200,"parent_record_offset":462176,"ancestor_record_offset":462176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":462368,"parent_record_offset":462344,"ancestor_record_offset":462176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":462344,"parent_record_offset":462320,"ancestor_record_offset":462176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":462320,"parent_record_offset":462296,"ancestor_record_offset":462176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":462296,"parent_record_offset":462272,"ancestor_record_offset":462176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":462224,"parent_record_offset":462200,"ancestor_record_offset":462176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D4
// 首条位置: {"module":1611,"inlinee":73940}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":460592,"parent_record_offset":460568,"ancestor_record_offset":460568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":460760,"parent_record_offset":460736,"ancestor_record_offset":460568,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":460736,"parent_record_offset":460712,"ancestor_record_offset":460568,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":460712,"parent_record_offset":460688,"ancestor_record_offset":460568,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":460688,"parent_record_offset":460664,"ancestor_record_offset":460568,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":460616,"parent_record_offset":460592,"ancestor_record_offset":460568,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D8
// 首条位置: {"module":1611,"inlinee":73944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":463808,"parent_record_offset":463784,"ancestor_record_offset":463784,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":463976,"parent_record_offset":463952,"ancestor_record_offset":463784,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":463952,"parent_record_offset":463928,"ancestor_record_offset":463784,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":463928,"parent_record_offset":463904,"ancestor_record_offset":463784,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":463904,"parent_record_offset":463880,"ancestor_record_offset":463784,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":463832,"parent_record_offset":463808,"ancestor_record_offset":463784,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12CE3
// 首条位置: {"module":1698,"inlinee":77027}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1698,"record_offset":132816,"parent_record_offset":132792,"ancestor_record_offset":132792,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1698,"record_offset":132984,"parent_record_offset":132960,"ancestor_record_offset":132792,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1698,"record_offset":132960,"parent_record_offset":132936,"ancestor_record_offset":132792,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1698,"record_offset":132936,"parent_record_offset":132912,"ancestor_record_offset":132792,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1698,"record_offset":132912,"parent_record_offset":132888,"ancestor_record_offset":132792,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1698,"record_offset":132840,"parent_record_offset":132816,"ancestor_record_offset":132792,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120D7
// 首条位置: {"module":1611,"inlinee":73943}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":463004,"parent_record_offset":462980,"ancestor_record_offset":462980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":463172,"parent_record_offset":463148,"ancestor_record_offset":462980,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1611,"record_offset":463148,"parent_record_offset":463124,"ancestor_record_offset":462980,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":463124,"parent_record_offset":463100,"ancestor_record_offset":462980,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1611,"record_offset":463100,"parent_record_offset":463076,"ancestor_record_offset":462980,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1611,"record_offset":463028,"parent_record_offset":463004,"ancestor_record_offset":462980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_chunk
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121BA
// 首条位置: {"module":1624,"inlinee":74170}
// 编译实例 RVAs: 0x17a6d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18609221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509744,"raw_hex":"e8bb0be4fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393509952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17830464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509744,"raw_hex":"e840121001","size":5,"soft_fail":false,"target_va":5411340320,"va":5393509851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17835937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509744,"raw_hex":"e8a1271001","size":5,"soft_fail":false,"target_va":5411345808,"va":5393509866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1624,"record_offset":14724,"parent_record_offset":14648,"ancestor_record_offset":14648,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":14780,"parent_record_offset":14648,"ancestor_record_offset":14648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":14696,"parent_record_offset":14648,"ancestor_record_offset":14648,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":14752,"parent_record_offset":14648,"ancestor_record_offset":14648,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 296
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1722,"record_offset":74644,"rva":24800848,"va":5393509968}
// 编译实例 RVAs: 0x17a6e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-10664205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8f3465dff","size":5,"soft_fail":false,"target_va":5382847888,"va":5393512088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10866529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e89f305aff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393511052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1158487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8a952eeff","size":5,"soft_fail":false,"target_va":5392353392,"va":5393511874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004f570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24482918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e89a6b8afe","size":5,"soft_fail":false,"target_va":5369034096,"va":5393517009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400519b0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24473523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e84d908afe","size":5,"soft_fail":false,"target_va":5369043376,"va":5393516894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23456287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e1159afe","size":5,"soft_fail":false,"target_va":5370060624,"va":5393516906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c8640
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21888713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e83701b2fe","size":5,"soft_fail":false,"target_va":5371627072,"va":5393515780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19887009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e85f8cd0fe","size":5,"soft_fail":false,"target_va":5373629488,"va":5393516492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18619160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e8e4e3fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5393519891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=8
//     target_aliases=3 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18610886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e83a05e4fe","size":5,"soft_fail":false,"target_va":5374904480,"va":5393515361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=38
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18428599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e849cde6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393510098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17994828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8b46bedfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393510855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085da90
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16036644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8dc4c0bff","size":5,"soft_fail":false,"target_va":5377481360,"va":5393517999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879b80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15920580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e83c120dff","size":5,"soft_fail":false,"target_va":5377596288,"va":5393516863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15898263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e869690dff","size":5,"soft_fail":false,"target_va":5377618720,"va":5393516978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15901197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8f35d0dff","size":5,"soft_fail":false,"target_va":5377619792,"va":5393520984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408825a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15889165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8f38c0dff","size":5,"soft_fail":false,"target_va":5377631648,"va":5393520808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15854417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8af140eff","size":5,"soft_fail":false,"target_va":5377658224,"va":5393512636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15788503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e829160fff","size":5,"soft_fail":false,"target_va":5377724224,"va":5393512722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa0a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13663806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8c2812fff","size":5,"soft_fail":false,"target_va":5379852944,"va":5393516745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10682673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8cffe5cff","size":5,"soft_fail":false,"target_va":5382827728,"va":5393510396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e6ad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4982016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e800fbb3ff","size":5,"soft_fail":false,"target_va":5388528336,"va":5393510347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f6160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4919660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e894eeb4ff","size":5,"soft_fail":false,"target_va":5388591456,"va":5393511111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f6e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e873e7b4ff","size":5,"soft_fail":false,"target_va":5388594704,"va":5393516184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e0e8b4ff","size":5,"soft_fail":false,"target_va":5388595504,"va":5393516619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f8270
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4912117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e80b0cb5ff","size":5,"soft_fail":false,"target_va":5388599920,"va":5393512032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141304390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4866966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e86abcb5ff","size":5,"soft_fail":false,"target_va":5388649360,"va":5393516321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3817882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e866bec5ff","size":5,"soft_fail":false,"target_va":5389693760,"va":5393511637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143c8c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3583517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e351c9ff","size":5,"soft_fail":false,"target_va":5389928640,"va":5393512152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14143d1c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3582355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e86d56c9ff","size":5,"soft_fail":false,"target_va":5389930944,"va":5393513294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414744d0
//     owner=unresolved_namespace; count=14
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3354960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8b0ceccff","size":5,"soft_fail":false,"target_va":5390157008,"va":5393511963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14157fff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2262523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8057addff","size":5,"soft_fail":false,"target_va":5391253488,"va":5393516006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159a120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2157402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8a614dfff","size":5,"soft_fail":false,"target_va":5391360288,"va":5393517685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159a8f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2149915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e531dfff","size":5,"soft_fail":false,"target_va":5391362288,"va":5393512198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159b080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2149097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e81735dfff","size":5,"soft_fail":false,"target_va":5391364224,"va":5393513316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159b850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2146700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8743edfff","size":5,"soft_fail":false,"target_va":5391366224,"va":5393512919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2148202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e89638dfff","size":5,"soft_fail":false,"target_va":5391367024,"va":5393515221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159c000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2146875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8c53ddfff","size":5,"soft_fail":false,"target_va":5391368192,"va":5393515062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159c7d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2143978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e81649dfff","size":5,"soft_fail":false,"target_va":5391370192,"va":5393514165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159cc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2144489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e81747dfff","size":5,"soft_fail":false,"target_va":5391371392,"va":5393515876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159e330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2138141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8e35fdfff","size":5,"soft_fail":false,"target_va":5391377200,"va":5393515336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159f930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2132038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8ba77dfff","size":5,"soft_fail":false,"target_va":5391382832,"va":5393514865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159ff30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2131186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e80e7bdfff","size":5,"soft_fail":false,"target_va":5391384368,"va":5393515549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a3140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2116561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e82fb4dfff","size":5,"soft_fail":false,"target_va":5391397184,"va":5393513740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2105975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e889dddfff","size":5,"soft_fail":false,"target_va":5391408624,"va":5393514594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2106034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e84edddfff","size":5,"soft_fail":false,"target_va":5391408864,"va":5393514893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2107651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8fdd6dfff","size":5,"soft_fail":false,"target_va":5391409120,"va":5393516766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-722524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8a4f9f4ff","size":5,"soft_fail":false,"target_va":5392788160,"va":5393510679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142080d70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9279697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8d1988d00","size":5,"soft_fail":false,"target_va":5402791280,"va":5393511578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17828473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8790a1001","size":5,"soft_fail":false,"target_va":5411340320,"va":5393511842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17828691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8530b1001","size":5,"soft_fail":false,"target_va":5411341280,"va":5393512584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9200
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17833112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8981c1001","size":5,"soft_fail":false,"target_va":5411344896,"va":5393511779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9590
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17835315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e833251001","size":5,"soft_fail":false,"target_va":5411345808,"va":5393510488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10670099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8ed2f5dff","size":5,"soft_fail":false,"target_va":5382840208,"va":5393510302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-10855098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8465d5aff","size":5,"soft_fail":false,"target_va":5382655024,"va":5393510117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18023229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e83d031301","size":5,"soft_fail":false,"target_va":5411541776,"va":5393518542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::unwrap_or<alloc::vec::Vec<tuple$<country_core::resources::walls::WallId,country_core::resources::color_id::ColorId>,alloc::alloc::Global>,serde_json::error::Error>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-32370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e88e81ffff","size":5,"soft_fail":false,"target_va":5393482400,"va":5393514765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::unwrap_or_default<std::collections::hash::set::HashSet<u64,std::hash::random::RandomState>,serde_json::error::Error>
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-33145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8877effff","size":5,"soft_fail":false,"target_va":5393482096,"va":5393515236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::closest_resolved_snapshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13771949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e853db2dff","size":5,"soft_fail":false,"target_va":5379738192,"va":5393510136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster_defs::AllWorldRasterWrite::ordered_names
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16161585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8cf6409ff","size":5,"soft_fail":false,"target_va":5377354512,"va":5393516092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster_defs::AllWorldRasterWrite::would_write_succeed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16159764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8ec6b09ff","size":5,"soft_fail":false,"target_va":5377356816,"va":5393516575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::snapshot::try_chunk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8fae4ffff","size":5,"soft_fail":false,"target_va":5393509744,"va":5393516657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] flate2::mem::Decompress::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12487771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8a57341ff","size":5,"soft_fail":false,"target_va":5381028928,"va":5393516694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1156593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e80f5aeeff","size":5,"soft_fail":false,"target_va":5392353952,"va":5393510540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12665183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8a1be3eff","size":5,"soft_fail":false,"target_va":5380845328,"va":5393510506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] motomoto::Reader::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"motomoto","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9279684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8c4988d00","size":5,"soft_fail":false,"target_va":5402791248,"va":5393511559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_json::value::impl$4::clone
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-32060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8c482ffff","size":5,"soft_fail":false,"target_va":5393482640,"va":5393514695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-10817030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8faf15aff","size":5,"soft_fail":false,"target_va":5382694656,"va":5393511681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10801801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e8772d5bff","size":5,"soft_fail":false,"target_va":5382708672,"va":5393510468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_starts_with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10768251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e885b05bff","size":5,"soft_fail":false,"target_va":5382742912,"va":5393511158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::unlink
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10739166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393509968,"raw_hex":"e822225cff","size":5,"soft_fail":false,"target_va":5382772112,"va":5393511273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*27177779(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5393509968,"raw_hex":"ff1533b39e01","size":6,"soft_fail":false,"va":5393510583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":91512,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":74844,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::remove_snapshot_record
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1722,"record_offset":92172,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":31776,"source_line":640,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::HistorySnapshotId::file_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1722,"record_offset":75724,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":31761,"source_line":283,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_load_snapshot_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1722,"record_offset":76232,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":77375,"source_line":327,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1722,"record_offset":91684,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":91428,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::fs::remove_file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":92712,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":8068,"source_line":2342,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1722,"record_offset":92340,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":21707,"source_line":2545,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1722,"record_offset":76092,"parent_record_offset":74644,"ancestor_record_offset":74644,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":75084,"parent_record_offset":75060,"ancestor_record_offset":74644,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":75060,"parent_record_offset":75036,"ancestor_record_offset":74644,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1722,"record_offset":75036,"parent_record_offset":75004,"ancestor_record_offset":74644,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":77936,"parent_record_offset":77848,"ancestor_record_offset":74644,"inlinee":6161,"source_line":673,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":77848,"parent_record_offset":77820,"ancestor_record_offset":74644,"inlinee":6160,"source_line":208,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1722,"record_offset":75748,"parent_record_offset":75724,"ancestor_record_offset":74644,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1722,"record_offset":75796,"parent_record_offset":75772,"ancestor_record_offset":74644,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78260,"parent_record_offset":78236,"ancestor_record_offset":74644,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":75984,"parent_record_offset":75956,"ancestor_record_offset":74644,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":75956,"parent_record_offset":75932,"ancestor_record_offset":74644,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78308,"parent_record_offset":78284,"ancestor_record_offset":74644,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78284,"parent_record_offset":78260,"ancestor_record_offset":74644,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":75932,"parent_record_offset":75908,"ancestor_record_offset":74644,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":78236,"parent_record_offset":78212,"ancestor_record_offset":74644,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82504,"parent_record_offset":82472,"ancestor_record_offset":74644,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":81324,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":83580,"parent_record_offset":83556,"ancestor_record_offset":74644,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":83296,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82184,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":78212,"parent_record_offset":78184,"ancestor_record_offset":74644,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82472,"parent_record_offset":82448,"ancestor_record_offset":74644,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1722,"record_offset":77488,"parent_record_offset":77448,"ancestor_record_offset":74644,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":77448,"parent_record_offset":77408,"ancestor_record_offset":74644,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":77408,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1722,"record_offset":84740,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":17634,"source_line":364,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":84764,"parent_record_offset":84740,"ancestor_record_offset":74644,"inlinee":14011,"source_line":266,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":84788,"parent_record_offset":84764,"ancestor_record_offset":74644,"inlinee":14012,"source_line":379,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1722,"record_offset":75208,"parent_record_offset":75176,"ancestor_record_offset":74644,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":83780,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":84100,"parent_record_offset":84072,"ancestor_record_offset":74644,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":84160,"parent_record_offset":84128,"ancestor_record_offset":74644,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1722,"record_offset":77320,"parent_record_offset":77244,"ancestor_record_offset":74644,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1722,"record_offset":84376,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1722,"record_offset":83400,"parent_record_offset":83376,"ancestor_record_offset":74644,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":83352,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1722,"record_offset":83496,"parent_record_offset":83468,"ancestor_record_offset":74644,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":84812,"parent_record_offset":84788,"ancestor_record_offset":74644,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":75232,"parent_record_offset":75208,"ancestor_record_offset":74644,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":78456,"parent_record_offset":78428,"ancestor_record_offset":74644,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":77888,"parent_record_offset":77848,"ancestor_record_offset":74644,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":75772,"parent_record_offset":75748,"ancestor_record_offset":74644,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":88844,"parent_record_offset":88820,"ancestor_record_offset":74644,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":77664,"parent_record_offset":77612,"ancestor_record_offset":74644,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":76896,"parent_record_offset":76872,"ancestor_record_offset":74644,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1722,"record_offset":91740,"parent_record_offset":91708,"ancestor_record_offset":74644,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1722,"record_offset":91768,"parent_record_offset":91708,"ancestor_record_offset":74644,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":84044,"parent_record_offset":84012,"ancestor_record_offset":74644,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=152
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":74876,"parent_record_offset":74844,"ancestor_record_offset":74644,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":82256,"parent_record_offset":82232,"ancestor_record_offset":74644,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":75592,"parent_record_offset":75568,"ancestor_record_offset":74644,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":75640,"parent_record_offset":75616,"ancestor_record_offset":74644,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":75616,"parent_record_offset":75592,"ancestor_record_offset":74644,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":82408,"parent_record_offset":82384,"ancestor_record_offset":74644,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":82332,"parent_record_offset":82296,"ancestor_record_offset":74644,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76984,"parent_record_offset":76840,"ancestor_record_offset":74644,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":88660,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":81980,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":83376,"parent_record_offset":83352,"ancestor_record_offset":74644,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76944,"parent_record_offset":76920,"ancestor_record_offset":74644,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76784,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":82208,"parent_record_offset":82184,"ancestor_record_offset":74644,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":83032,"parent_record_offset":83008,"ancestor_record_offset":74644,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1722,"record_offset":82232,"parent_record_offset":82208,"ancestor_record_offset":74644,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82384,"parent_record_offset":82360,"ancestor_record_offset":74644,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82944,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":83008,"parent_record_offset":82944,"ancestor_record_offset":74644,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82296,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82360,"parent_record_offset":82296,"ancestor_record_offset":74644,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":83720,"parent_record_offset":83696,"ancestor_record_offset":74644,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":83744,"parent_record_offset":83720,"ancestor_record_offset":74644,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_removed_supports::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_removed_supports.rs
//     evidence={"module":1722,"record_offset":82152,"parent_record_offset":82128,"ancestor_record_offset":74644,"inlinee":33443,"source_line":8,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::StairGraphEdge::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":82860,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":28265,"source_line":56,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":81948,"parent_record_offset":81924,"ancestor_record_offset":74644,"inlinee":33603,"source_line":516,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1722,"record_offset":81460,"parent_record_offset":81436,"ancestor_record_offset":74644,"inlinee":28090,"source_line":43,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::TerrainHeightsData::as_raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1722,"record_offset":84640,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":30722,"source_line":321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1722,"record_offset":83096,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1722,"record_offset":80296,"parent_record_offset":80272,"ancestor_record_offset":74644,"inlinee":74032,"source_line":1234,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1722,"record_offset":77244,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":74170,"source_line":286,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1722,"record_offset":84708,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":20311,"source_line":168,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":84072,"parent_record_offset":84012,"ancestor_record_offset":74644,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":84128,"parent_record_offset":84072,"ancestor_record_offset":74644,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":83664,"parent_record_offset":83640,"ancestor_record_offset":74644,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":83868,"parent_record_offset":83840,"ancestor_record_offset":74644,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":84012,"parent_record_offset":83924,"ancestor_record_offset":74644,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83896,"parent_record_offset":83868,"ancestor_record_offset":74644,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83984,"parent_record_offset":83924,"ancestor_record_offset":74644,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83956,"parent_record_offset":83924,"ancestor_record_offset":74644,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83924,"parent_record_offset":83896,"ancestor_record_offset":74644,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":85384,"parent_record_offset":85360,"ancestor_record_offset":74644,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1722,"record_offset":82912,"parent_record_offset":82888,"ancestor_record_offset":74644,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":1722,"record_offset":84248,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4881,"source_line":2348,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":91796,"parent_record_offset":91708,"ancestor_record_offset":74644,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1722,"record_offset":91708,"parent_record_offset":91684,"ancestor_record_offset":74644,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":91820,"parent_record_offset":91796,"ancestor_record_offset":74644,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":83696,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76812,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":13581,"source_line":579,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76872,"parent_record_offset":76840,"ancestor_record_offset":74644,"inlinee":13584,"source_line":228,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76840,"parent_record_offset":76812,"ancestor_record_offset":74644,"inlinee":13583,"source_line":338,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76920,"parent_record_offset":76896,"ancestor_record_offset":74644,"inlinee":13586,"source_line":229,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":79472,"parent_record_offset":79448,"ancestor_record_offset":74644,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":78000,"parent_record_offset":77976,"ancestor_record_offset":74644,"inlinee":13338,"source_line":134,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$89::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":81808,"parent_record_offset":81784,"ancestor_record_offset":74644,"inlinee":27196,"source_line":1005,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$2::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":77820,"parent_record_offset":77696,"ancestor_record_offset":74644,"inlinee":11756,"source_line":302,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$12::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1722,"record_offset":77764,"parent_record_offset":77696,"ancestor_record_offset":74644,"inlinee":15117,"source_line":21,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$18::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1722,"record_offset":77788,"parent_record_offset":77764,"ancestor_record_offset":74644,"inlinee":33489,"source_line":27,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::from_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1722,"record_offset":77976,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":19006,"source_line":982,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::impl$4::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1722,"record_offset":77696,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":11754,"source_line":114,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1722,"record_offset":77636,"parent_record_offset":77612,"ancestor_record_offset":74644,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1722,"record_offset":77612,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83640,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83840,"parent_record_offset":83812,"ancestor_record_offset":74644,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83812,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1722,"record_offset":82888,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1722,"record_offset":76140,"parent_record_offset":76116,"ancestor_record_offset":74644,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1722,"record_offset":76188,"parent_record_offset":76164,"ancestor_record_offset":74644,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1722,"record_offset":76164,"parent_record_offset":76140,"ancestor_record_offset":74644,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76668,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76692,"parent_record_offset":76668,"ancestor_record_offset":74644,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76752,"parent_record_offset":76668,"ancestor_record_offset":74644,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1722,"record_offset":84852,"parent_record_offset":76232,"ancestor_record_offset":74644,"inlinee":21396,"source_line":977,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1722,"record_offset":75568,"parent_record_offset":74940,"ancestor_record_offset":74644,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1722,"record_offset":74940,"parent_record_offset":74908,"ancestor_record_offset":74644,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1722,"record_offset":77140,"parent_record_offset":77112,"ancestor_record_offset":74644,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1722,"record_offset":76116,"parent_record_offset":76092,"ancestor_record_offset":74644,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1722,"record_offset":92768,"parent_record_offset":92744,"ancestor_record_offset":74644,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1722,"record_offset":92744,"parent_record_offset":92712,"ancestor_record_offset":74644,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1722,"record_offset":76720,"parent_record_offset":76692,"ancestor_record_offset":74644,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12E3F
// 首条位置: {"module":1722,"inlinee":77375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1722,"record_offset":79364,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":81324,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":83296,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82184,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":77408,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":1722,"record_offset":84740,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":17634,"source_line":364,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":83780,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1722,"record_offset":84376,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":83352,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":77028,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":88660,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":5961,"source_line":1439,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":81980,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":77216,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76784,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82944,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82296,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::StairGraphEdge::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":82860,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":28265,"source_line":56,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::as_raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1722,"record_offset":84640,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":30722,"source_line":321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1722,"record_offset":83096,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1722,"record_offset":77244,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":74170,"source_line":286,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1722,"record_offset":84708,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":20311,"source_line":168,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::collect_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":1722,"record_offset":84248,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4881,"source_line":2348,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":83696,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76812,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":13581,"source_line":579,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::from_value
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1722,"record_offset":77976,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":19006,"source_line":982,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::impl$4::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/mod.rs
//     evidence={"module":1722,"record_offset":77696,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":11754,"source_line":114,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::index::impl$5::index
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1722,"record_offset":77612,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":26177,"source_line":212,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83640,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":4955,"source_line":605,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83812,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1722,"record_offset":82888,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76668,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1722,"record_offset":84852,"parent_record_offset":76232,"ancestor_record_offset":76232,"inlinee":21396,"source_line":977,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":78512,"parent_record_offset":78488,"ancestor_record_offset":76232,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":78488,"parent_record_offset":78400,"ancestor_record_offset":76232,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1722,"record_offset":84976,"parent_record_offset":84952,"ancestor_record_offset":76232,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::new_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":77936,"parent_record_offset":77848,"ancestor_record_offset":76232,"inlinee":6161,"source_line":673,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$2::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1722,"record_offset":77848,"parent_record_offset":77820,"ancestor_record_offset":76232,"inlinee":6160,"source_line":208,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1722,"record_offset":79412,"parent_record_offset":79388,"ancestor_record_offset":76232,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78260,"parent_record_offset":78236,"ancestor_record_offset":76232,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78428,"parent_record_offset":78400,"ancestor_record_offset":76232,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78400,"parent_record_offset":78376,"ancestor_record_offset":76232,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78308,"parent_record_offset":78284,"ancestor_record_offset":76232,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78284,"parent_record_offset":78260,"ancestor_record_offset":76232,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":78376,"parent_record_offset":78352,"ancestor_record_offset":76232,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":78236,"parent_record_offset":78212,"ancestor_record_offset":76232,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82504,"parent_record_offset":82472,"ancestor_record_offset":76232,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":83580,"parent_record_offset":83556,"ancestor_record_offset":76232,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":78212,"parent_record_offset":78184,"ancestor_record_offset":76232,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":82472,"parent_record_offset":82448,"ancestor_record_offset":76232,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1722,"record_offset":77488,"parent_record_offset":77448,"ancestor_record_offset":76232,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1722,"record_offset":77448,"parent_record_offset":77408,"ancestor_record_offset":76232,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":84764,"parent_record_offset":84740,"ancestor_record_offset":76232,"inlinee":14011,"source_line":266,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":1722,"record_offset":84788,"parent_record_offset":84764,"ancestor_record_offset":76232,"inlinee":14012,"source_line":379,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":84100,"parent_record_offset":84072,"ancestor_record_offset":76232,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":84160,"parent_record_offset":84128,"ancestor_record_offset":76232,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1722,"record_offset":77320,"parent_record_offset":77244,"ancestor_record_offset":76232,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1722,"record_offset":83400,"parent_record_offset":83376,"ancestor_record_offset":76232,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1722,"record_offset":84276,"parent_record_offset":84248,"ancestor_record_offset":76232,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1722,"record_offset":83496,"parent_record_offset":83468,"ancestor_record_offset":76232,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1722,"record_offset":84812,"parent_record_offset":84788,"ancestor_record_offset":76232,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":78456,"parent_record_offset":78428,"ancestor_record_offset":76232,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":77888,"parent_record_offset":77848,"ancestor_record_offset":76232,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":79388,"parent_record_offset":79364,"ancestor_record_offset":76232,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":88844,"parent_record_offset":88820,"ancestor_record_offset":76232,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":77664,"parent_record_offset":77612,"ancestor_record_offset":76232,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1722,"record_offset":76896,"parent_record_offset":76872,"ancestor_record_offset":76232,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":84044,"parent_record_offset":84012,"ancestor_record_offset":76232,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":77056,"parent_record_offset":77028,"ancestor_record_offset":76232,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":82256,"parent_record_offset":82232,"ancestor_record_offset":76232,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":82408,"parent_record_offset":82384,"ancestor_record_offset":76232,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":82332,"parent_record_offset":82296,"ancestor_record_offset":76232,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76984,"parent_record_offset":76840,"ancestor_record_offset":76232,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":83376,"parent_record_offset":83352,"ancestor_record_offset":76232,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":76944,"parent_record_offset":76920,"ancestor_record_offset":76232,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1722,"record_offset":77348,"parent_record_offset":77244,"ancestor_record_offset":76232,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":82208,"parent_record_offset":82184,"ancestor_record_offset":76232,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":83032,"parent_record_offset":83008,"ancestor_record_offset":76232,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1722,"record_offset":82232,"parent_record_offset":82208,"ancestor_record_offset":76232,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82384,"parent_record_offset":82360,"ancestor_record_offset":76232,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":83008,"parent_record_offset":82944,"ancestor_record_offset":76232,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1722,"record_offset":82360,"parent_record_offset":82296,"ancestor_record_offset":76232,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":83720,"parent_record_offset":83696,"ancestor_record_offset":76232,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":83744,"parent_record_offset":83720,"ancestor_record_offset":76232,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_removed_supports::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_removed_supports.rs
//     evidence={"module":1722,"record_offset":82152,"parent_record_offset":82128,"ancestor_record_offset":76232,"inlinee":33443,"source_line":8,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1722,"record_offset":81948,"parent_record_offset":81924,"ancestor_record_offset":76232,"inlinee":33603,"source_line":516,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":1722,"record_offset":81460,"parent_record_offset":81436,"ancestor_record_offset":76232,"inlinee":28090,"source_line":43,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":1722,"record_offset":80296,"parent_record_offset":80272,"ancestor_record_offset":76232,"inlinee":74032,"source_line":1234,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":84072,"parent_record_offset":84012,"ancestor_record_offset":76232,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":84128,"parent_record_offset":84072,"ancestor_record_offset":76232,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":83664,"parent_record_offset":83640,"ancestor_record_offset":76232,"inlinee":4956,"source_line":845,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":83868,"parent_record_offset":83840,"ancestor_record_offset":76232,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":84012,"parent_record_offset":83924,"ancestor_record_offset":76232,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83896,"parent_record_offset":83868,"ancestor_record_offset":76232,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83984,"parent_record_offset":83924,"ancestor_record_offset":76232,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83956,"parent_record_offset":83924,"ancestor_record_offset":76232,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":83924,"parent_record_offset":83896,"ancestor_record_offset":76232,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":85384,"parent_record_offset":85360,"ancestor_record_offset":76232,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1722,"record_offset":82912,"parent_record_offset":82888,"ancestor_record_offset":76232,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76872,"parent_record_offset":76840,"ancestor_record_offset":76232,"inlinee":13584,"source_line":228,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76840,"parent_record_offset":76812,"ancestor_record_offset":76232,"inlinee":13583,"source_line":338,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":1722,"record_offset":76920,"parent_record_offset":76896,"ancestor_record_offset":76232,"inlinee":13586,"source_line":229,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":79472,"parent_record_offset":79448,"ancestor_record_offset":76232,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$54::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":78000,"parent_record_offset":77976,"ancestor_record_offset":76232,"inlinee":13338,"source_line":134,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$89::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1722,"record_offset":81808,"parent_record_offset":81784,"ancestor_record_offset":76232,"inlinee":27196,"source_line":1005,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$2::clone
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1722,"record_offset":77820,"parent_record_offset":77696,"ancestor_record_offset":76232,"inlinee":11756,"source_line":302,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$12::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1722,"record_offset":77764,"parent_record_offset":77696,"ancestor_record_offset":76232,"inlinee":15117,"source_line":21,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::number::impl$18::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/number.rs
//     evidence={"module":1722,"record_offset":77788,"parent_record_offset":77764,"ancestor_record_offset":76232,"inlinee":33489,"source_line":27,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/index.rs
//     evidence={"module":1722,"record_offset":77636,"parent_record_offset":77612,"ancestor_record_offset":76232,"inlinee":26179,"source_line":123,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":83840,"parent_record_offset":83812,"ancestor_record_offset":76232,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76692,"parent_record_offset":76668,"ancestor_record_offset":76232,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1722,"record_offset":76752,"parent_record_offset":76668,"ancestor_record_offset":76232,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":1722,"record_offset":77140,"parent_record_offset":77112,"ancestor_record_offset":76232,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1722,"record_offset":76720,"parent_record_offset":76692,"ancestor_record_offset":76232,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121B8
// 首条位置: {"module":1624,"inlinee":74168}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":14984,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1624,"record_offset":15416,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":1624,"record_offset":14840,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":40637,"source_line":321,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":15532,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":15728,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::snapshot::try_chunk
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/snapshot.rs
//     evidence={"module":1624,"record_offset":14648,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":74170,"source_line":286,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] flate2::deflate::bufread::DeflateDecoder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"flate2","version":"1.0.24","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/flate2-1.0.24/src/deflate/bufread.rs
//     evidence={"module":1624,"record_offset":15356,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":20311,"source_line":168,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] std::io::Read::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\mod.rs
//     evidence={"module":1624,"record_offset":15388,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":21396,"source_line":977,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1624,"record_offset":14812,"parent_record_offset":14544,"ancestor_record_offset":14544,"inlinee":21628,"source_line":98,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":14544,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":14544,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":14544,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":15252,"parent_record_offset":15148,"ancestor_record_offset":14544,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":14544,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1624,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":14544,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15064,"parent_record_offset":15024,"ancestor_record_offset":14544,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15900,"parent_record_offset":15868,"ancestor_record_offset":14544,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15868,"parent_record_offset":15840,"ancestor_record_offset":14544,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15148,"parent_record_offset":15104,"ancestor_record_offset":14544,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15104,"parent_record_offset":15064,"ancestor_record_offset":14544,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15840,"parent_record_offset":15812,"ancestor_record_offset":14544,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":15196,"parent_record_offset":15148,"ancestor_record_offset":14544,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1624,"record_offset":15024,"parent_record_offset":14984,"ancestor_record_offset":14544,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1624,"record_offset":15492,"parent_record_offset":15452,"ancestor_record_offset":14544,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1624,"record_offset":15452,"parent_record_offset":15416,"ancestor_record_offset":14544,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1624,"record_offset":15220,"parent_record_offset":15196,"ancestor_record_offset":14544,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1624,"record_offset":14724,"parent_record_offset":14648,"ancestor_record_offset":14544,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1624,"record_offset":14940,"parent_record_offset":14916,"ancestor_record_offset":14544,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":14780,"parent_record_offset":14648,"ancestor_record_offset":14544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":14696,"parent_record_offset":14648,"ancestor_record_offset":14544,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":14752,"parent_record_offset":14648,"ancestor_record_offset":14544,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1624,"record_offset":14916,"parent_record_offset":14888,"ancestor_record_offset":14544,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":1624,"record_offset":14864,"parent_record_offset":14840,"ancestor_record_offset":14544,"inlinee":21967,"source_line":360,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":1624,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":14544,"inlinee":20267,"source_line":347,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::try_load_snapshot_data::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x121B4
// 首条位置: {"module":1624,"inlinee":74164}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1624,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":6144,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":6144,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":6436,"parent_record_offset":6380,"ancestor_record_offset":6144,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":6408,"parent_record_offset":6380,"ancestor_record_offset":6144,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":6144,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":6144,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":6284,"parent_record_offset":6192,"ancestor_record_offset":6144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1624,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":6144,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1624,"record_offset":6224,"parent_record_offset":6192,"ancestor_record_offset":6144,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1624,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":6144,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::snapshot::init_game_state_from_snapshot
// 记录形式: compiled_procedure; instances=1
// 源码行: 470
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1722,"record_offset":94932,"rva":24812624,"va":5393521744}
// 编译实例 RVAs: 0x17a9c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10877555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e88d055aff","size":5,"soft_fail":false,"target_va":5382644528,"va":5393522078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004f570
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24490240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8004f8afe","size":5,"soft_fail":false,"target_va":5369034096,"va":5393524331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23466769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8efec99fe","size":5,"soft_fail":false,"target_va":5370060624,"va":5393527388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18441590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e88a9ae6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393523089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18005861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e89b40edfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5393521888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15931003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e885e90cff","size":5,"soft_fail":false,"target_va":5377596288,"va":5393527286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15908655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8d1400dff","size":5,"soft_fail":false,"target_va":5377618720,"va":5393527370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087f750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15907287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e829460dff","size":5,"soft_fail":false,"target_va":5377619792,"va":5393527074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140880a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15902201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8075a0dff","size":5,"soft_fail":false,"target_va":5377624672,"va":5393526868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408825a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15895480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e848740dff","size":5,"soft_fail":false,"target_va":5377631648,"va":5393527123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: snapshot.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18325060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8449e1701","size":5,"soft_fail":false,"target_va":5411847168,"va":5393522103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-19167686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e83a86dbfe","size":5,"soft_fail":false,"target_va":5374355376,"va":5393523057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18017295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e80fec1201","size":5,"soft_fail":false,"target_va":5411543264,"va":5393525964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::restore_snapshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-15381984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8204a15ff","size":5,"soft_fail":false,"target_va":5378143632,"va":5393525611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::restore_snapshot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14193832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8586b27ff","size":5,"soft_fail":false,"target_va":5379329392,"va":5393523219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::load_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16052930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e83e0d0bff","size":5,"soft_fail":false,"target_va":5377469824,"va":5393522749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::world_raster_defs::AllWorldRasterWrite::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16169915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393521744,"raw_hex":"e8454409ff","size":5,"soft_fail":false,"target_va":5377354720,"va":5393524630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":101336,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":95424,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1722,"record_offset":96220,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1722,"record_offset":95552,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1722,"record_offset":100812,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1722,"record_offset":95216,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":95904,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1722,"record_offset":101268,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeightsData::as_mut_raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1722,"record_offset":101232,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":30723,"source_line":325,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1722,"record_offset":95248,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":95132,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":98708,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":12667,"source_line":378,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":99420,"parent_record_offset":94932,"ancestor_record_offset":94932,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":96160,"parent_record_offset":96136,"ancestor_record_offset":94932,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1722,"record_offset":96136,"parent_record_offset":96048,"ancestor_record_offset":94932,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":95744,"parent_record_offset":95712,"ancestor_record_offset":94932,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":96076,"parent_record_offset":96048,"ancestor_record_offset":94932,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":96048,"parent_record_offset":96024,"ancestor_record_offset":94932,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":95792,"parent_record_offset":95768,"ancestor_record_offset":94932,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":95768,"parent_record_offset":95744,"ancestor_record_offset":94932,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1722,"record_offset":96024,"parent_record_offset":96000,"ancestor_record_offset":94932,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":95712,"parent_record_offset":95668,"ancestor_record_offset":94932,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":95668,"parent_record_offset":95624,"ancestor_record_offset":94932,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1722,"record_offset":103472,"parent_record_offset":103448,"ancestor_record_offset":94932,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":95928,"parent_record_offset":95904,"ancestor_record_offset":94932,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1722,"record_offset":95976,"parent_record_offset":95952,"ancestor_record_offset":94932,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1722,"record_offset":95868,"parent_record_offset":95580,"ancestor_record_offset":94932,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1722,"record_offset":100836,"parent_record_offset":100812,"ancestor_record_offset":94932,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1722,"record_offset":95580,"parent_record_offset":95552,"ancestor_record_offset":94932,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1722,"record_offset":100892,"parent_record_offset":100812,"ancestor_record_offset":94932,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1722,"record_offset":95624,"parent_record_offset":95580,"ancestor_record_offset":94932,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":98920,"parent_record_offset":98892,"ancestor_record_offset":94932,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1722,"record_offset":98980,"parent_record_offset":98948,"ancestor_record_offset":94932,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1722,"record_offset":101304,"parent_record_offset":101268,"ancestor_record_offset":94932,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":100656,"parent_record_offset":100628,"ancestor_record_offset":94932,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1722,"record_offset":96104,"parent_record_offset":96076,"ancestor_record_offset":94932,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1722,"record_offset":99712,"parent_record_offset":99680,"ancestor_record_offset":94932,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1722,"record_offset":99680,"parent_record_offset":99648,"ancestor_record_offset":94932,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1722,"record_offset":95304,"parent_record_offset":95272,"ancestor_record_offset":94932,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1722,"record_offset":95332,"parent_record_offset":95272,"ancestor_record_offset":94932,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1722,"record_offset":99016,"parent_record_offset":98864,"ancestor_record_offset":94932,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":95952,"parent_record_offset":95928,"ancestor_record_offset":94932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":101360,"parent_record_offset":101336,"ancestor_record_offset":94932,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1722,"record_offset":99972,"parent_record_offset":99948,"ancestor_record_offset":94932,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":95492,"parent_record_offset":95424,"ancestor_record_offset":94932,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1722,"record_offset":95460,"parent_record_offset":95424,"ancestor_record_offset":94932,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":95516,"parent_record_offset":95492,"ancestor_record_offset":94932,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1722,"record_offset":95832,"parent_record_offset":95668,"ancestor_record_offset":94932,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":95156,"parent_record_offset":95132,"ancestor_record_offset":94932,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1722,"record_offset":95180,"parent_record_offset":95156,"ancestor_record_offset":94932,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1722,"record_offset":99608,"parent_record_offset":99568,"ancestor_record_offset":94932,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1722,"record_offset":99648,"parent_record_offset":99608,"ancestor_record_offset":94932,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1722,"record_offset":99756,"parent_record_offset":99568,"ancestor_record_offset":94932,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1722,"record_offset":99568,"parent_record_offset":99520,"ancestor_record_offset":94932,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":98892,"parent_record_offset":98864,"ancestor_record_offset":94932,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":98948,"parent_record_offset":98892,"ancestor_record_offset":94932,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1722,"record_offset":99848,"parent_record_offset":99520,"ancestor_record_offset":94932,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":98756,"parent_record_offset":98732,"ancestor_record_offset":94932,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1722,"record_offset":99468,"parent_record_offset":99444,"ancestor_record_offset":94932,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":99948,"parent_record_offset":99520,"ancestor_record_offset":94932,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":98864,"parent_record_offset":98804,"ancestor_record_offset":94932,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":99520,"parent_record_offset":99492,"ancestor_record_offset":94932,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":98780,"parent_record_offset":98756,"ancestor_record_offset":94932,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100596,"parent_record_offset":100564,"ancestor_record_offset":94932,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":98836,"parent_record_offset":98804,"ancestor_record_offset":94932,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100508,"parent_record_offset":100484,"ancestor_record_offset":94932,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100564,"parent_record_offset":100508,"ancestor_record_offset":94932,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100536,"parent_record_offset":100508,"ancestor_record_offset":94932,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":98804,"parent_record_offset":98780,"ancestor_record_offset":94932,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100628,"parent_record_offset":100596,"ancestor_record_offset":94932,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":100260,"parent_record_offset":100236,"ancestor_record_offset":94932,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1722,"record_offset":99492,"parent_record_offset":99468,"ancestor_record_offset":94932,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":95360,"parent_record_offset":95272,"ancestor_record_offset":94932,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1722,"record_offset":95272,"parent_record_offset":95248,"ancestor_record_offset":94932,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1722,"record_offset":95384,"parent_record_offset":95360,"ancestor_record_offset":94932,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":98732,"parent_record_offset":98708,"ancestor_record_offset":94932,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01722.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1722,"record_offset":99444,"parent_record_offset":99420,"ancestor_record_offset":94932,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01722.jsonl.gz"}
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
// 完整结构及原始类型记录: snapshot.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot::LoadedSnapshotData
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::snapshot::LoadedSnapshotData>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::snapshot::LoadedSnapshotData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot::SnapshotLoadResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "country_core::systems::save::snapshot::SnapshotLoadResources", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >", "tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> > > >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >,country_core::systems::GameSystemSet>", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::in_set", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >", "country_core::systems::GameSystemSet"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::snapshot::SnapshotSaveData
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::snapshot::SnapshotSaveData>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::snapshot::SnapshotSaveData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::systems::save::snapshot::SnapshotSaveData,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::systems::save::snapshot::SnapshotSaveData", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::systems::save::snapshot::SnapshotSaveData,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::systems::save::snapshot::SnapshotSaveData,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapshot.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapshot.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 227; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "country_core::systems::save::snapshot::SnapshotLoadResources", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >", "tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref_mut$<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData>,ref$<country_core::resources::walls::curved_halftimber::CurvedHalfTimber> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemTerrain>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_draw_terrain::CacheMouseRaycastHeight>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::get_param
// 信息 ID: 3614fc85945172dd83746430
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0cf452bde58780bc66d5e0be: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >; module=1611, inline_record=2673004; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >; module=1698, inline_record=20904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::init_state
// 信息 ID: 3e9264dbb0f755e4154d701e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0cf452bde58780bc66d5e0be: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > @ 0x1749401 -> 0x17ab290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param
// 信息 ID: e7b90cbf7f1ee4ea4bbbc5d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0cf452bde58780bc66d5e0be: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> > >; module=1611, inline_record=1392480; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,country_core::systems::save::snapshot::SnapshotLoadResources,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref_mut$<country_core::systems::debug::sheep_controls::SheepControl>,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep> >,tuple$<bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning> >)> >; module=1698, inline_record=99728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$10
// 信息 ID: 4e176dcd6f05c8d0e006c88f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10>; module=1611, inline_record=461372; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$10>; module=1698, inline_record=131184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$11
// 信息 ID: 7c91507055c8ba34032f937e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11>; module=1611, inline_record=464588; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$11>; module=1698, inline_record=135204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$12
// 信息 ID: ff5c5b5d4a4f2247562ddf73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12>; module=1611, inline_record=462176; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$12>; module=1698, inline_record=131988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$13
// 信息 ID: 5089c344e80cc271677f26d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13>; module=1611, inline_record=460568; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$13>; module=1698, inline_record=130380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$14
// 信息 ID: f0e17bd4cc703841bd80843e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14>; module=1611, inline_record=463784; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$14>; module=1698, inline_record=134400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$8
// 信息 ID: ae6a3e2e25d1dd46b459e704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$8 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$8>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$8>; module=1698, inline_record=132792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::_::impl$0::validate_param::closure$9
// 信息 ID: 68f5e46cec6fe205766b9b94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "country_core::systems::save::snapshot::_::impl$0::validate_param", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9 <- core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9>; module=1611, inline_record=462980; 不是独立调用边。
// [PDB 内联宿主] core::result::Result::map_err<tuple$<>,bevy_ecs::system::system_param::SystemParamValidationError,bevy_ecs::system::system_param::SystemParamValidationError,country_core::systems::save::snapshot::_::impl$0::validate_param::closure_env$9>; module=1698, inline_record=133596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot
// 信息 ID: 33a72a60d157c86771854f27
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot_save_data
// 信息 ID: 704ee674b1b9bec0dae459d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_game::save_game @ 0xa40132 -> 0x17a0430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$0
// 信息 ID: e62fe103e986a8a10254d794
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::snapshot::capture_game_snapshot_save_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,core::iter::adapters::map::Map<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::next<tuple$<country_core::components::TreeStorageIndex,country_core::resources::history::tree_edit::TreeData>,bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$0>; module=1585, inline_record=17960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$1
// 信息 ID: 1d7b71e613a34f07d086e8b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::snapshot::capture_game_snapshot_save_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState>,country_core::resources::walls::inner_walls::impl$1::all::closure_env$0>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<country_core::resources::walls::WallId,ref$<country_core::resources::walls::inner_wall_state::WallState> > >,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$1>; module=554, inline_record=600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure$2
// 信息 ID: 405a174a6546e74e2671ce1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::save::snapshot::capture_game_snapshot_save_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2 <- core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,alloc::vec::Vec<u8,alloc::alloc::Global> >,alloc::alloc::Global,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple$<ref$<str$>,ref$<slice2$<u8> > >,6>,country_core::systems::save::snapshot::capture_game_snapshot_save_data::closure_env$2> > > >; module=1610, inline_record=71132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::init_game_state_from_snapshot
// 信息 ID: 0ce73e8963cacc2496febb4e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x146f7e6 -> 0x17a9c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_chunk
// 信息 ID: 1b3feccff56112373bb07072
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data @ 0x17a8871 -> 0x17a6d70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1624, inline_record=14648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data; module=1722, inline_record=77244; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data
// 信息 ID: c3bad7b1a5fc274b8b48e6c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data; module=1722, inline_record=76232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data::closure$0
// 信息 ID: 4a810d7de6b84dceee1d16ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::snapshot::try_load_snapshot_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- hashbrown::map::impl$82::extend<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- alloc::vec::into_iter::impl$5::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- alloc::vec::into_iter::impl$5::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- alloc::vec::into_iter::impl$5::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$25::from_iter::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0> >,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,core::result::impl$25::from_iter::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0> >,std::collections::hash::map::HashMap<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<alloc::string::String,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_try_fold::closure_env$0<alloc::string::String,enum2$<core::result::Result<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,anyhow::Error> >,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > >,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::ops::try_trait::impl$0::wrap_mut_2::closure_env$0<tuple$<>,tuple$<>,tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::string::String,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > > > >,core::ops::try_trait::NeverShortCircuit<tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<core::ops::try_trait::NeverShortCircuit<tuple$<> >,tuple$<> > > >; module=1624, inline_record=14544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::try_load_snapshot_data::closure$1
// 信息 ID: 450f54faa6433d4c1288faa9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::snapshot::try_load_snapshot_data", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref$<str$>,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1 <- alloc::vec::into_iter::impl$5::fold<ref$<str$>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<str$>,tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,tuple$<>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref$<str$>,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1 <- alloc::vec::into_iter::impl$5::fold<ref$<str$>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<str$>,tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,tuple$<>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref$<str$>,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::fold<ref$<str$>,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ref$<str$>,tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,tuple$<>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::vec::impl$20::extend_trusted::closure_env$0<tuple$<ref$<str$>,enum2$<core::option::Option<alloc::vec::Vec<u8,alloc::alloc::Global> > > >,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ref$<str$>,alloc::alloc::Global>,country_core::systems::save::snapshot::try_load_snapshot_data::closure_env$1> > > > >; module=1624, inline_record=6144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::snapshot::validate_and_load_closest_snapshot_data
// 信息 ID: e09a451ede1b4735e94d049f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x146df6e -> 0x17a6e50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
