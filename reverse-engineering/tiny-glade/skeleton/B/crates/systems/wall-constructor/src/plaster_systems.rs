// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/plaster_systems.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3cb4f9d495647dd42f109318f73b661fe9f4c04405551e06110c33f75d2c7c18
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: plaster_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 395, 1410, 1411, 1415, 1417, 1443, 1444, 3024, 4061, 4067, 4068
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":395,"record_offset":2248,"rva":7714832,"va":5376423952}
// 编译实例 RVAs: 0x75b810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7446116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e89c618eff","size":5,"soft_fail":false,"target_va":5368983552,"va":5376429663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=5
//     target_aliases=7 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1748762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8e650e5ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376425093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1616798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e86254e7ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5376431113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1343849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8977eebff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376425348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140885fb0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1216018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8128e1200","size":5,"soft_fail":false,"target_va":5377646512,"va":5376430489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5680
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1477795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8a38c1600","size":5,"soft_fail":false,"target_va":5377906304,"va":5376428504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2878663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8c7ec2b00","size":5,"soft_fail":false,"target_va":5379305728,"va":5376427060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b777f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4307793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e851bb4100","size":5,"soft_fail":false,"target_va":5380732912,"va":5376425114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5508076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8ec0b5400","size":5,"soft_fail":false,"target_va":5381934560,"va":5376426479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f86cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8561199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e82fa28200","size":5,"soft_fail":false,"target_va":5384989904,"va":5376428700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fcad0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11143179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e80b08aa00","size":5,"soft_fail":false,"target_va":5387569872,"va":5376426688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141200ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11162519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e89753aa00","size":5,"soft_fail":false,"target_va":5387587264,"va":5376424740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412036d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11171579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8fb76aa00","size":5,"soft_fail":false,"target_va":5387597520,"va":5376425936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141205b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11179652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e88496aa00","size":5,"soft_fail":false,"target_va":5387606784,"va":5376427127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412269b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11312840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8c89eac00","size":5,"soft_fail":false,"target_va":5387741616,"va":5376428771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122ca20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11337086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e87efdac00","size":5,"soft_fail":false,"target_va":5387766304,"va":5376429213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2070417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e86f68e0ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5376426556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5998848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e800895b00","size":5,"soft_fail":false,"target_va":5382424432,"va":5376425579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35112307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e873c51702","size":5,"soft_fail":false,"target_va":5411541584,"va":5376429272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35112983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e817c81702","size":5,"soft_fail":false,"target_va":5411541776,"va":5376428788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35113719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8f7ca1702","size":5,"soft_fail":false,"target_va":5411542880,"va":5376429156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11346765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e84d23ad00","size":5,"soft_fail":false,"target_va":5387776144,"va":5376429374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e848fbffff","size":5,"soft_fail":false,"target_va":5376422896,"va":5376424099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35119186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e852e01702","size":5,"soft_fail":false,"target_va":5411543360,"va":5376424169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3990609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e851e43c00","size":5,"soft_fail":false,"target_va":5380417056,"va":5376426442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3990637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e86de43c00","size":5,"soft_fail":false,"target_va":5380417024,"va":5376426382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8561687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e817a48200","size":5,"soft_fail":false,"target_va":5384990384,"va":5376428692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11153657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8f930aa00","size":5,"soft_fail":false,"target_va":5387580064,"va":5376426402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4960933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8a5b24b00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376424598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4966122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8eac64b00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376424513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4957574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e886a54b00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376428901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4960637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e87db14b00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376424062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35109527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e897ba1702","size":5,"soft_fail":false,"target_va":5411538608,"va":5376429076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35109697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e841bb1702","size":5,"soft_fail":false,"target_va":5411538768,"va":5376429066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35109785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e899bb1702","size":5,"soft_fail":false,"target_va":5411538960,"va":5376429170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35211766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8f6491902","size":5,"soft_fail":false,"target_va":5411640768,"va":5376428997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35108045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e8cdb41702","size":5,"soft_fail":false,"target_va":5411537104,"va":5376429054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5456772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376423952,"raw_hex":"e884435300","size":5,"soft_fail":false,"target_va":5381883104,"va":5376426327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":395,"record_offset":15928,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":23377,"source_line":352,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":22932,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":10753,"source_line":3172,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":12964,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":16108,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":395,"record_offset":16164,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":395,"record_offset":7164,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":6764,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":395,"record_offset":6064,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":395,"record_offset":12556,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":395,"record_offset":9432,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":395,"record_offset":12468,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":395,"record_offset":7372,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":395,"record_offset":11068,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":395,"record_offset":11012,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":12496,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":21532,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$7::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":7344,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":24530,"source_line":2088,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":9492,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":3924,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":2500,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":2948,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":11456,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":12936,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":11124,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":7224,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":14040,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":17336,"source_line":705,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":13144,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":4908,"parent_record_offset":2248,"ancestor_record_offset":2248,"inlinee":24528,"source_line":169,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":2248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":8032,"parent_record_offset":7944,"ancestor_record_offset":2248,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":12748,"parent_record_offset":12716,"ancestor_record_offset":2248,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":7972,"parent_record_offset":7944,"ancestor_record_offset":2248,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":2248,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":2248,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":2248,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":2248,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":395,"record_offset":15964,"parent_record_offset":15928,"ancestor_record_offset":2248,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":12716,"parent_record_offset":12676,"ancestor_record_offset":2248,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5524,"parent_record_offset":5500,"ancestor_record_offset":2248,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":2248,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":22964,"parent_record_offset":22932,"ancestor_record_offset":2248,"inlinee":10754,"source_line":2264,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":23024,"parent_record_offset":22964,"ancestor_record_offset":2248,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":12676,"parent_record_offset":12640,"ancestor_record_offset":2248,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":23248,"parent_record_offset":22964,"ancestor_record_offset":2248,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":11092,"parent_record_offset":11068,"ancestor_record_offset":2248,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5476,"parent_record_offset":4908,"ancestor_record_offset":2248,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":23052,"parent_record_offset":22964,"ancestor_record_offset":2248,"inlinee":10756,"source_line":3173,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":6384,"parent_record_offset":6152,"ancestor_record_offset":2248,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":2248,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":395,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":2248,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":395,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":2248,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":395,"record_offset":7568,"parent_record_offset":7544,"ancestor_record_offset":2248,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":395,"record_offset":7192,"parent_record_offset":7164,"ancestor_record_offset":2248,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":395,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":2248,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":395,"record_offset":12640,"parent_record_offset":12580,"ancestor_record_offset":2248,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":395,"record_offset":6128,"parent_record_offset":6088,"ancestor_record_offset":2248,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":2248,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":2248,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":2248,"inlinee":15366,"source_line":90,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::reborrow
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":395,"record_offset":9460,"parent_record_offset":9432,"ancestor_record_offset":2248,"inlinee":13700,"source_line":266,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":395,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":2248,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4260,"parent_record_offset":4232,"ancestor_record_offset":2248,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4312,"parent_record_offset":4288,"ancestor_record_offset":2248,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4288,"parent_record_offset":4260,"ancestor_record_offset":2248,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4208,"parent_record_offset":4180,"ancestor_record_offset":2248,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2248,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4404,"parent_record_offset":4232,"ancestor_record_offset":2248,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":2248,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":2248,"inlinee":10758,"source_line":1976,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":2248,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":11312,"parent_record_offset":11284,"ancestor_record_offset":2248,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":11708,"parent_record_offset":11684,"ancestor_record_offset":2248,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":11372,"parent_record_offset":11340,"ancestor_record_offset":2248,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":395,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":2248,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":395,"record_offset":12208,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":2248,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":2248,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":2248,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":395,"record_offset":7464,"parent_record_offset":7400,"ancestor_record_offset":2248,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":395,"record_offset":7400,"parent_record_offset":7372,"ancestor_record_offset":2248,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":395,"record_offset":8156,"parent_record_offset":8124,"ancestor_record_offset":2248,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":395,"record_offset":8124,"parent_record_offset":7400,"ancestor_record_offset":2248,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":395,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":2248,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":395,"record_offset":8220,"parent_record_offset":8156,"ancestor_record_offset":2248,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":395,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":2248,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":395,"record_offset":4336,"parent_record_offset":4312,"ancestor_record_offset":2248,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":2248,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":6200,"parent_record_offset":6152,"ancestor_record_offset":2248,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":8000,"parent_record_offset":7972,"ancestor_record_offset":2248,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":15068,"parent_record_offset":14992,"ancestor_record_offset":2248,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":395,"record_offset":2648,"parent_record_offset":2616,"ancestor_record_offset":2248,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":15268,"parent_record_offset":15240,"ancestor_record_offset":2248,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":15296,"parent_record_offset":15240,"ancestor_record_offset":2248,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":11940,"parent_record_offset":11912,"ancestor_record_offset":2248,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":11912,"parent_record_offset":11884,"ancestor_record_offset":2248,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":395,"record_offset":4076,"parent_record_offset":4032,"ancestor_record_offset":2248,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":2248,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":7720,"parent_record_offset":7464,"ancestor_record_offset":2248,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":2248,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":7100,"parent_record_offset":6788,"ancestor_record_offset":2248,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":6900,"parent_record_offset":6820,"ancestor_record_offset":2248,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":2248,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":2248,"inlinee":11885,"source_line":2240,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$33::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":2248,"inlinee":11884,"source_line":2374,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":2248,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":4864,"parent_record_offset":4032,"ancestor_record_offset":2248,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":15404,"parent_record_offset":15380,"ancestor_record_offset":2248,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":4468,"parent_record_offset":4180,"ancestor_record_offset":2248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":2248,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":2248,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":7048,"parent_record_offset":7024,"ancestor_record_offset":2248,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2248,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":7656,"parent_record_offset":7568,"ancestor_record_offset":2248,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2248,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":2248,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":2248,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":2248,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":395,"record_offset":5436,"parent_record_offset":4908,"ancestor_record_offset":2248,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":395,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":2248,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":395,"record_offset":15612,"parent_record_offset":15524,"ancestor_record_offset":2248,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":2248,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":6256,"parent_record_offset":6152,"ancestor_record_offset":2248,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":6608,"parent_record_offset":6152,"ancestor_record_offset":2248,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":2248,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":2248,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":395,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":2248,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2248,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":2248,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2248,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":3616,"parent_record_offset":3588,"ancestor_record_offset":2248,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":6976,"parent_record_offset":6924,"ancestor_record_offset":2248,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":3264,"parent_record_offset":3148,"ancestor_record_offset":2248,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":2248,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":5560,"parent_record_offset":4908,"ancestor_record_offset":2248,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":395,"record_offset":16000,"parent_record_offset":15964,"ancestor_record_offset":2248,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":395,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":2248,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":395,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2248,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":395,"record_offset":3116,"parent_record_offset":3084,"ancestor_record_offset":2248,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":395,"record_offset":3084,"parent_record_offset":3048,"ancestor_record_offset":2248,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2248,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2248,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2248,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":3588,"parent_record_offset":3048,"ancestor_record_offset":2248,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2588,"parent_record_offset":2556,"ancestor_record_offset":2248,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2248,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2748,"parent_record_offset":2716,"ancestor_record_offset":2248,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2844,"parent_record_offset":2748,"ancestor_record_offset":2248,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2248,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2532,"parent_record_offset":2500,"ancestor_record_offset":2248,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":2248,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":3148,"parent_record_offset":3116,"ancestor_record_offset":2248,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":3432,"parent_record_offset":3148,"ancestor_record_offset":2248,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":3404,"parent_record_offset":3148,"ancestor_record_offset":2248,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":3208,"parent_record_offset":3148,"ancestor_record_offset":2248,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":2248,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5900,"parent_record_offset":5712,"ancestor_record_offset":2248,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5048,"parent_record_offset":5016,"ancestor_record_offset":2248,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":2248,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":2248,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":2248,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":2248,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":2248,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":2248,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":395,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":2248,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":395,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":2248,"inlinee":24536,"source_line":436,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":15240,"parent_record_offset":14992,"ancestor_record_offset":2248,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":2248,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":11884,"parent_record_offset":11852,"ancestor_record_offset":2248,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":395,"record_offset":13468,"parent_record_offset":13280,"ancestor_record_offset":2248,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":11824,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":11684,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":11284,"parent_record_offset":11256,"ancestor_record_offset":2248,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":12236,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":11340,"parent_record_offset":11284,"ancestor_record_offset":2248,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":395,"record_offset":13560,"parent_record_offset":13228,"ancestor_record_offset":2248,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":11768,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":395,"record_offset":12388,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":395,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":2248,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":11488,"parent_record_offset":11456,"ancestor_record_offset":2248,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":11520,"parent_record_offset":11488,"ancestor_record_offset":2248,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":11148,"parent_record_offset":11124,"ancestor_record_offset":2248,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":2248,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":2248,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::retain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":14064,"parent_record_offset":14040,"ancestor_record_offset":2248,"inlinee":17919,"source_line":917,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":12092,"parent_record_offset":11984,"ancestor_record_offset":2248,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":2248,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":7304,"parent_record_offset":7276,"ancestor_record_offset":2248,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":2248,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":2248,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":2248,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":13660,"parent_record_offset":13228,"ancestor_record_offset":2248,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":15356,"parent_record_offset":15332,"ancestor_record_offset":2248,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":12328,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":11256,"parent_record_offset":11196,"ancestor_record_offset":2248,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":13228,"parent_record_offset":13192,"ancestor_record_offset":2248,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":19980,"parent_record_offset":19956,"ancestor_record_offset":2248,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":15332,"parent_record_offset":14960,"ancestor_record_offset":2248,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":14932,"parent_record_offset":14064,"ancestor_record_offset":2248,"inlinee":17923,"source_line":774,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":14960,"parent_record_offset":14932,"ancestor_record_offset":2248,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":11592,"parent_record_offset":11560,"ancestor_record_offset":2248,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":11560,"parent_record_offset":11520,"ancestor_record_offset":2248,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":11172,"parent_record_offset":11148,"ancestor_record_offset":2248,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":11228,"parent_record_offset":11196,"ancestor_record_offset":2248,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":14992,"parent_record_offset":14960,"ancestor_record_offset":2248,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":11628,"parent_record_offset":11592,"ancestor_record_offset":2248,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":11196,"parent_record_offset":11172,"ancestor_record_offset":2248,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":12360,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":15036,"parent_record_offset":14992,"ancestor_record_offset":2248,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":2248,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":11984,"parent_record_offset":11628,"ancestor_record_offset":2248,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":2248,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":3960,"parent_record_offset":3924,"ancestor_record_offset":2248,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":21940,"parent_record_offset":21908,"ancestor_record_offset":2248,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":22456,"parent_record_offset":22196,"ancestor_record_offset":2248,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":395,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":2248,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":4180,"parent_record_offset":4032,"ancestor_record_offset":2248,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":4440,"parent_record_offset":4180,"ancestor_record_offset":2248,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":395,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":2248,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":395,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":2248,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5712,"parent_record_offset":5672,"ancestor_record_offset":2248,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5328,"parent_record_offset":5136,"ancestor_record_offset":2248,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":2248,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":4976,"parent_record_offset":4908,"ancestor_record_offset":2248,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":5108,"parent_record_offset":4976,"ancestor_record_offset":2248,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":5672,"parent_record_offset":5640,"ancestor_record_offset":2248,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":395,"record_offset":5016,"parent_record_offset":4976,"ancestor_record_offset":2248,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":395,"record_offset":5960,"parent_record_offset":5672,"ancestor_record_offset":2248,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":395,"record_offset":4132,"parent_record_offset":4104,"ancestor_record_offset":2248,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":395,"record_offset":4032,"parent_record_offset":3996,"ancestor_record_offset":2248,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":395,"record_offset":3996,"parent_record_offset":3960,"ancestor_record_offset":2248,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":395,"record_offset":5172,"parent_record_offset":5136,"ancestor_record_offset":2248,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":2248,"inlinee":24532,"source_line":99,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":15452,"parent_record_offset":14064,"ancestor_record_offset":2248,"inlinee":24533,"source_line":122,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$6
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":21472,"parent_record_offset":21448,"ancestor_record_offset":2248,"inlinee":24534,"source_line":153,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11535
// 首条位置: {"module":1443,"inlinee":70965}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD4
// 首条位置: {"module":395,"inlinee":24532}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD5
// 首条位置: {"module":395,"inlinee":24533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15452,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":15580,"parent_record_offset":15552,"ancestor_record_offset":15452,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":395,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":15452,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":395,"record_offset":15612,"parent_record_offset":15524,"ancestor_record_offset":15452,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":15452,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":15552,"parent_record_offset":15524,"ancestor_record_offset":15452,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":395,"record_offset":15636,"parent_record_offset":15612,"ancestor_record_offset":15452,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=80
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11542
// 首条位置: {"module":1444,"inlinee":70978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD6
// 首条位置: {"module":395,"inlinee":24534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":395,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":21472,"inlinee":24536,"source_line":436,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FD0
// 首条位置: {"module":395,"inlinee":24528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5476,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":5640,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":395,"record_offset":5436,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":5560,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":4976,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5524,"parent_record_offset":5500,"ancestor_record_offset":4908,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":5500,"parent_record_offset":5476,"ancestor_record_offset":4908,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":5604,"parent_record_offset":5560,"ancestor_record_offset":4908,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":4908,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5900,"parent_record_offset":5712,"ancestor_record_offset":4908,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5048,"parent_record_offset":5016,"ancestor_record_offset":4908,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":4908,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":4908,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4908,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":4908,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":4908,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":4908,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":395,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":4908,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":395,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4908,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5712,"parent_record_offset":5672,"ancestor_record_offset":4908,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5328,"parent_record_offset":5136,"ancestor_record_offset":4908,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":4908,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":5108,"parent_record_offset":4976,"ancestor_record_offset":4908,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":395,"record_offset":5672,"parent_record_offset":5640,"ancestor_record_offset":4908,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":395,"record_offset":5016,"parent_record_offset":4976,"ancestor_record_offset":4908,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":395,"record_offset":5960,"parent_record_offset":5672,"ancestor_record_offset":4908,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":395,"record_offset":5172,"parent_record_offset":5136,"ancestor_record_offset":4908,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11461
// 首条位置: {"module":1415,"inlinee":70753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::impl$2::register_required_components
// 记录形式: compiled_procedure; instances=1
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":395,"record_offset":36764,"rva":7727584,"va":5376436704}
// 编译实例 RVAs: 0x75e9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140051340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7396232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e878248fff","size":5,"soft_fail":false,"target_va":5369041728,"va":5376437955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1499664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8f01de9ff","size":5,"soft_fail":false,"target_va":5374937936,"va":5376437595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1356523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8154debff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376438022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14075e9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e861fbffff","size":5,"soft_fail":false,"target_va":5376436704,"va":5376437882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141200a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11148656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8701daa00","size":5,"soft_fail":false,"target_va":5387586192,"va":5376437531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35409587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8b34e1c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376437576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4337356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8cc2e4200","size":5,"soft_fail":false,"target_va":5380775408,"va":5376438047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::ComponentInfo::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6163312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e8700b5e00","size":5,"soft_fail":false,"target_va":5382600816,"va":5376437499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::QueuedRegistration::register
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6163100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e89c0a5e00","size":5,"soft_fail":false,"target_va":5382601280,"va":5376438175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::component::enforce_no_required_components_recursion
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6169403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e83b235e00","size":5,"soft_fail":false,"target_va":5382606192,"va":5376436784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTable$LT$T$C$A$GT$::reserve_rehash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35073885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376436704,"raw_hex":"e85d2f1702","size":5,"soft_fail":false,"target_va":5411512256,"va":5376438366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":42988,"parent_record_offset":36764,"ancestor_record_offset":36764,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":42792,"parent_record_offset":36764,"ancestor_record_offset":36764,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::component::ComponentsRegistrator::register_component
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":36936,"parent_record_offset":36764,"ancestor_record_offset":36764,"inlinee":5611,"source_line":1715,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":42504,"parent_record_offset":42480,"ancestor_record_offset":36764,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":42480,"parent_record_offset":42388,"ancestor_record_offset":36764,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":41924,"parent_record_offset":41900,"ancestor_record_offset":36764,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":42416,"parent_record_offset":42388,"ancestor_record_offset":36764,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":42388,"parent_record_offset":42364,"ancestor_record_offset":36764,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":41972,"parent_record_offset":41948,"ancestor_record_offset":36764,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":41948,"parent_record_offset":41924,"ancestor_record_offset":36764,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":42364,"parent_record_offset":42340,"ancestor_record_offset":36764,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":41900,"parent_record_offset":41876,"ancestor_record_offset":36764,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":41876,"parent_record_offset":41852,"ancestor_record_offset":36764,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":36980,"parent_record_offset":36936,"ancestor_record_offset":36764,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":41852,"parent_record_offset":39848,"ancestor_record_offset":36764,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentDescriptor::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":39916,"parent_record_offset":39848,"ancestor_record_offset":36764,"inlinee":5647,"source_line":1101,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentIds::next_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":39816,"parent_record_offset":37008,"ancestor_record_offset":36764,"inlinee":5644,"source_line":1334,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::register_component_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":39944,"parent_record_offset":39848,"ancestor_record_offset":36764,"inlinee":5648,"source_line":1972,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_checked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":37008,"parent_record_offset":36936,"ancestor_record_offset":36764,"inlinee":5612,"source_line":1721,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::ComponentsRegistrator::register_component_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":39848,"parent_record_offset":37008,"ancestor_record_offset":36764,"inlinee":5645,"source_line":1755,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$17::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":40080,"parent_record_offset":39848,"ancestor_record_offset":36764,"inlinee":13108,"source_line":1614,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$90::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":395,"record_offset":41824,"parent_record_offset":39848,"ancestor_record_offset":36764,"inlinee":5662,"source_line":2812,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":395,"record_offset":37076,"parent_record_offset":37008,"ancestor_record_offset":36764,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":395,"record_offset":38116,"parent_record_offset":37008,"ancestor_record_offset":36764,"inlinee":5624,"source_line":1141,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":395,"record_offset":37856,"parent_record_offset":37832,"ancestor_record_offset":36764,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":37904,"parent_record_offset":37880,"ancestor_record_offset":36764,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_cmpeq_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":39376,"parent_record_offset":39348,"ancestor_record_offset":36764,"inlinee":5636,"source_line":842,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":41264,"parent_record_offset":41240,"ancestor_record_offset":36764,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":37304,"parent_record_offset":37280,"ancestor_record_offset":36764,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":37388,"parent_record_offset":37364,"ancestor_record_offset":36764,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":395,"record_offset":37328,"parent_record_offset":37304,"ancestor_record_offset":36764,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":42448,"parent_record_offset":42416,"ancestor_record_offset":36764,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":39236,"parent_record_offset":39188,"ancestor_record_offset":36764,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":39612,"parent_record_offset":39584,"ancestor_record_offset":36764,"inlinee":5639,"source_line":135,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":39640,"parent_record_offset":39584,"ancestor_record_offset":36764,"inlinee":5640,"source_line":161,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":37540,"parent_record_offset":37512,"ancestor_record_offset":36764,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":37512,"parent_record_offset":37484,"ancestor_record_offset":36764,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":40640,"parent_record_offset":40276,"ancestor_record_offset":36764,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":40664,"parent_record_offset":40640,"ancestor_record_offset":36764,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":39148,"parent_record_offset":39124,"ancestor_record_offset":36764,"inlinee":5634,"source_line":682,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":40048,"parent_record_offset":39944,"ancestor_record_offset":36764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":39736,"parent_record_offset":39712,"ancestor_record_offset":36764,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":37772,"parent_record_offset":37748,"ancestor_record_offset":36764,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":41752,"parent_record_offset":41728,"ancestor_record_offset":36764,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":39760,"parent_record_offset":39736,"ancestor_record_offset":36764,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":41776,"parent_record_offset":41752,"ancestor_record_offset":36764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":39992,"parent_record_offset":39944,"ancestor_record_offset":36764,"inlinee":5649,"source_line":619,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":40016,"parent_record_offset":39992,"ancestor_record_offset":36764,"inlinee":5650,"source_line":232,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":38048,"parent_record_offset":38024,"ancestor_record_offset":36764,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":395,"record_offset":38072,"parent_record_offset":38048,"ancestor_record_offset":36764,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":395,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":36764,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":395,"record_offset":37832,"parent_record_offset":37808,"ancestor_record_offset":36764,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::leading_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":39584,"parent_record_offset":39188,"ancestor_record_offset":36764,"inlinee":5637,"source_line":86,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":37448,"parent_record_offset":37420,"ancestor_record_offset":36764,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":37484,"parent_record_offset":37448,"ancestor_record_offset":36764,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":37420,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":37280,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":41240,"parent_record_offset":41180,"ancestor_record_offset":36764,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":37584,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":40728,"parent_record_offset":40700,"ancestor_record_offset":36764,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":37364,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":395,"record_offset":41516,"parent_record_offset":41484,"ancestor_record_offset":36764,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":40164,"parent_record_offset":40120,"ancestor_record_offset":36764,"inlinee":5653,"source_line":1804,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":37100,"parent_record_offset":37076,"ancestor_record_offset":36764,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":37124,"parent_record_offset":37100,"ancestor_record_offset":36764,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":40120,"parent_record_offset":40080,"ancestor_record_offset":36764,"inlinee":5652,"source_line":1790,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":38148,"parent_record_offset":38116,"ancestor_record_offset":36764,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":38180,"parent_record_offset":38148,"ancestor_record_offset":36764,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":37808,"parent_record_offset":37700,"ancestor_record_offset":36764,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37748,"parent_record_offset":37724,"ancestor_record_offset":36764,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39712,"parent_record_offset":39020,"ancestor_record_offset":36764,"inlinee":5642,"source_line":488,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::to_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39100,"parent_record_offset":39076,"ancestor_record_offset":36764,"inlinee":5631,"source_line":334,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41728,"parent_record_offset":41452,"ancestor_record_offset":36764,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37668,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37724,"parent_record_offset":37700,"ancestor_record_offset":36764,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39076,"parent_record_offset":39048,"ancestor_record_offset":36764,"inlinee":5630,"source_line":705,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::erase_no_drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39048,"parent_record_offset":39020,"ancestor_record_offset":36764,"inlinee":5629,"source_line":766,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37176,"parent_record_offset":37152,"ancestor_record_offset":36764,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find_or_find_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":40200,"parent_record_offset":40164,"ancestor_record_offset":36764,"inlinee":5654,"source_line":1109,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37152,"parent_record_offset":37124,"ancestor_record_offset":36764,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_in_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41452,"parent_record_offset":40120,"ancestor_record_offset":36764,"inlinee":5665,"source_line":1145,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39020,"parent_record_offset":38204,"ancestor_record_offset":36764,"inlinee":5628,"source_line":785,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":38204,"parent_record_offset":38180,"ancestor_record_offset":36764,"inlinee":5627,"source_line":797,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":40240,"parent_record_offset":40200,"ancestor_record_offset":36764,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::erase
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39188,"parent_record_offset":39048,"ancestor_record_offset":36764,"inlinee":5635,"source_line":3062,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":36764,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":40700,"parent_record_offset":40276,"ancestor_record_offset":36764,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_or_find_insert_slot_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":40276,"parent_record_offset":40200,"ancestor_record_offset":36764,"inlinee":5656,"source_line":1648,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41180,"parent_record_offset":40276,"ancestor_record_offset":36764,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41212,"parent_record_offset":41180,"ancestor_record_offset":36764,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37252,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::record_item_insert_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41484,"parent_record_offset":41452,"ancestor_record_offset":36764,"inlinee":5666,"source_line":2311,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39672,"parent_record_offset":39188,"ancestor_record_offset":36764,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":41552,"parent_record_offset":41484,"ancestor_record_offset":36764,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":42116,"parent_record_offset":42092,"ancestor_record_offset":36764,"inlinee":4995,"source_line":3323,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":37700,"parent_record_offset":37200,"ancestor_record_offset":36764,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find_or_find_insert_slot::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":40912,"parent_record_offset":40276,"ancestor_record_offset":36764,"inlinee":5660,"source_line":1127,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::offset_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":39124,"parent_record_offset":39100,"ancestor_record_offset":36764,"inlinee":5633,"source_line":18,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":38000,"parent_record_offset":37976,"ancestor_record_offset":36764,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":395,"record_offset":38024,"parent_record_offset":38000,"ancestor_record_offset":36764,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":395,"record_offset":37976,"parent_record_offset":37008,"ancestor_record_offset":36764,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::impl$0::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1411,"record_offset":65764,"rva":18867712,"va":5387576832}
// 编译实例 RVAs: 0x11fe600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18593909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e88b47e4fe","size":5,"soft_fail":false,"target_va":5368983552,"va":5387577456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12763246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e8923f3dff","size":5,"soft_fail":false,"target_va":5374814320,"va":5387577561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6407262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e8a23b9eff","size":5,"soft_fail":false,"target_va":5381170352,"va":5387577609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13221237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e88b4236ff","size":5,"soft_fail":false,"target_va":5374355952,"va":5387577184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e805fdffff","size":5,"soft_fail":false,"target_va":5387576448,"va":5387577206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23965505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e841af6d01","size":5,"soft_fail":false,"target_va":5411542880,"va":5387577370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23961312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e8e09e6d01","size":5,"soft_fail":false,"target_va":5411538608,"va":5387577291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23961488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e8909f6d01","size":5,"soft_fail":false,"target_va":5411538768,"va":5387577275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23961571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387576832,"raw_hex":"e8e39f6d01","size":5,"soft_fail":false,"target_va":5411538960,"va":5387577384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":66468,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":65932,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":67104,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1411,"record_offset":66788,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1411,"record_offset":66760,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1411,"record_offset":67416,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1411,"record_offset":66424,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1411,"record_offset":65960,"parent_record_offset":65764,"ancestor_record_offset":65764,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1411,"record_offset":66528,"parent_record_offset":66504,"ancestor_record_offset":65764,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1411,"record_offset":66576,"parent_record_offset":66552,"ancestor_record_offset":65764,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1411,"record_offset":66552,"parent_record_offset":66528,"ancestor_record_offset":65764,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":67152,"parent_record_offset":67104,"ancestor_record_offset":65764,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":66504,"parent_record_offset":66468,"ancestor_record_offset":65764,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":66616,"parent_record_offset":66468,"ancestor_record_offset":65764,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1411,"record_offset":66644,"parent_record_offset":66468,"ancestor_record_offset":65764,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1411,"record_offset":67032,"parent_record_offset":67004,"ancestor_record_offset":65764,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1411,"record_offset":67004,"parent_record_offset":66788,"ancestor_record_offset":65764,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1411,"record_offset":67300,"parent_record_offset":67104,"ancestor_record_offset":65764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1411,"record_offset":66720,"parent_record_offset":66696,"ancestor_record_offset":65764,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1411,"record_offset":66964,"parent_record_offset":66940,"ancestor_record_offset":65764,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1411,"record_offset":66880,"parent_record_offset":66824,"ancestor_record_offset":65764,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1411,"record_offset":67060,"parent_record_offset":67032,"ancestor_record_offset":65764,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1411,"record_offset":67272,"parent_record_offset":67104,"ancestor_record_offset":65764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1411,"record_offset":66672,"parent_record_offset":66468,"ancestor_record_offset":65764,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1411,"record_offset":66696,"parent_record_offset":66672,"ancestor_record_offset":65764,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1411,"record_offset":66940,"parent_record_offset":66916,"ancestor_record_offset":65764,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1411,"record_offset":66824,"parent_record_offset":66788,"ancestor_record_offset":65764,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1411,"record_offset":66916,"parent_record_offset":66824,"ancestor_record_offset":65764,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":66344,"parent_record_offset":66316,"ancestor_record_offset":65764,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":67668,"parent_record_offset":67480,"ancestor_record_offset":65764,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":66032,"parent_record_offset":66000,"ancestor_record_offset":65764,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":67760,"parent_record_offset":67728,"ancestor_record_offset":65764,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":66248,"parent_record_offset":66216,"ancestor_record_offset":65764,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":66056,"parent_record_offset":66032,"ancestor_record_offset":65764,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":66272,"parent_record_offset":66248,"ancestor_record_offset":65764,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":67692,"parent_record_offset":67668,"ancestor_record_offset":65764,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1411,"record_offset":67784,"parent_record_offset":67760,"ancestor_record_offset":65764,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1411,"record_offset":66216,"parent_record_offset":66188,"ancestor_record_offset":65764,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":1411,"record_offset":66188,"parent_record_offset":66160,"ancestor_record_offset":65764,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1411,"record_offset":67480,"parent_record_offset":67444,"ancestor_record_offset":65764,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1411,"record_offset":66316,"parent_record_offset":66120,"ancestor_record_offset":65764,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":1411,"record_offset":66120,"parent_record_offset":66092,"ancestor_record_offset":65764,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1411,"record_offset":66092,"parent_record_offset":65960,"ancestor_record_offset":65764,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":1411,"record_offset":67444,"parent_record_offset":67416,"ancestor_record_offset":65764,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1411,"record_offset":66000,"parent_record_offset":65960,"ancestor_record_offset":65764,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1411,"record_offset":67728,"parent_record_offset":67444,"ancestor_record_offset":65764,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":1411,"record_offset":66160,"parent_record_offset":66120,"ancestor_record_offset":65764,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/01411.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":395,"record_offset":24680,"rva":7722080,"va":5376431200}
// 编译实例 RVAs: 0x75d460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1755722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e8b635e5ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376432053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f86cd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8557452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e88c938200","size":5,"soft_fail":false,"target_va":5384989904,"va":5376432447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412269b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11308894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e85e8fac00","size":5,"soft_fail":false,"target_va":5387741616,"va":5376432717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35108649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e829b71702","size":5,"soft_fail":false,"target_va":5411541584,"va":5376432930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35109011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e893b81702","size":5,"soft_fail":false,"target_va":5411541776,"va":5376432760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e831dfffff","size":5,"soft_fail":false,"target_va":5376422896,"va":5376431290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35111995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e83bc41702","size":5,"soft_fail":false,"target_va":5411543360,"va":5376431360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8557940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e874958200","size":5,"soft_fail":false,"target_va":5384990384,"va":5376432439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4953736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e888964b00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376431795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4958922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e8caaa4b00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376431713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4953656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e838964b00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376432819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4953444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e864954b00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376431255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35207901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e8dd3a1902","size":5,"soft_fail":false,"target_va":5411640768,"va":5376432862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35104181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376431200,"raw_hex":"e8b5a51702","size":5,"soft_fail":false,"target_va":5411537104,"va":5376432918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":27948,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":395,"record_offset":27248,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":31360,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":31444,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":31472,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":26280,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":24856,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":25304,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":30352,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":395,"record_offset":28288,"parent_record_offset":24680,"ancestor_record_offset":24680,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":27592,"parent_record_offset":27568,"ancestor_record_offset":24680,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":395,"record_offset":27568,"parent_record_offset":27336,"ancestor_record_offset":24680,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":395,"record_offset":27312,"parent_record_offset":27272,"ancestor_record_offset":24680,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":27336,"parent_record_offset":27312,"ancestor_record_offset":24680,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":395,"record_offset":27972,"parent_record_offset":27948,"ancestor_record_offset":24680,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":395,"record_offset":27272,"parent_record_offset":27248,"ancestor_record_offset":24680,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26600,"parent_record_offset":26572,"ancestor_record_offset":24680,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26652,"parent_record_offset":26628,"ancestor_record_offset":24680,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26628,"parent_record_offset":26600,"ancestor_record_offset":24680,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26548,"parent_record_offset":26520,"ancestor_record_offset":24680,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26572,"parent_record_offset":26548,"ancestor_record_offset":24680,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26744,"parent_record_offset":26572,"ancestor_record_offset":24680,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":395,"record_offset":26856,"parent_record_offset":26832,"ancestor_record_offset":24680,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":24680,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":24680,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":395,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":24680,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":395,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":24680,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":395,"record_offset":29140,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":28024,"parent_record_offset":28000,"ancestor_record_offset":24680,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":28000,"parent_record_offset":27972,"ancestor_record_offset":24680,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":395,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":24680,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":395,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":24680,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":27524,"parent_record_offset":27496,"ancestor_record_offset":24680,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":27384,"parent_record_offset":27336,"ancestor_record_offset":24680,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":395,"record_offset":25004,"parent_record_offset":24972,"ancestor_record_offset":24680,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":28812,"parent_record_offset":28784,"ancestor_record_offset":24680,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":395,"record_offset":28784,"parent_record_offset":28756,"ancestor_record_offset":24680,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":395,"record_offset":26416,"parent_record_offset":26376,"ancestor_record_offset":24680,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":28048,"parent_record_offset":28024,"ancestor_record_offset":24680,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":28080,"parent_record_offset":28000,"ancestor_record_offset":24680,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":25252,"parent_record_offset":25224,"ancestor_record_offset":24680,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":395,"record_offset":27204,"parent_record_offset":26376,"ancestor_record_offset":24680,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":26808,"parent_record_offset":26520,"ancestor_record_offset":24680,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":27840,"parent_record_offset":27816,"ancestor_record_offset":24680,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":28932,"parent_record_offset":28908,"ancestor_record_offset":24680,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":28228,"parent_record_offset":28204,"ancestor_record_offset":24680,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":25716,"parent_record_offset":25692,"ancestor_record_offset":24680,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":25692,"parent_record_offset":25668,"ancestor_record_offset":24680,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":395,"record_offset":25900,"parent_record_offset":25872,"ancestor_record_offset":24680,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":26700,"parent_record_offset":26676,"ancestor_record_offset":24680,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":27440,"parent_record_offset":27336,"ancestor_record_offset":24680,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":395,"record_offset":27792,"parent_record_offset":27336,"ancestor_record_offset":24680,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":27496,"parent_record_offset":27468,"ancestor_record_offset":24680,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":395,"record_offset":27468,"parent_record_offset":27440,"ancestor_record_offset":24680,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":395,"record_offset":27816,"parent_record_offset":27792,"ancestor_record_offset":24680,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":25644,"parent_record_offset":25620,"ancestor_record_offset":24680,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":28204,"parent_record_offset":28180,"ancestor_record_offset":24680,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":25668,"parent_record_offset":25644,"ancestor_record_offset":24680,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":25972,"parent_record_offset":25944,"ancestor_record_offset":24680,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":28156,"parent_record_offset":28104,"ancestor_record_offset":24680,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":25620,"parent_record_offset":25504,"ancestor_record_offset":24680,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":395,"record_offset":28180,"parent_record_offset":28156,"ancestor_record_offset":24680,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":395,"record_offset":24972,"parent_record_offset":24944,"ancestor_record_offset":24680,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":395,"record_offset":25332,"parent_record_offset":25304,"ancestor_record_offset":24680,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":395,"record_offset":25472,"parent_record_offset":25440,"ancestor_record_offset":24680,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":395,"record_offset":25440,"parent_record_offset":25404,"ancestor_record_offset":24680,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":25380,"parent_record_offset":25356,"ancestor_record_offset":24680,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":25404,"parent_record_offset":25380,"ancestor_record_offset":24680,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":25356,"parent_record_offset":25332,"ancestor_record_offset":24680,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":395,"record_offset":25944,"parent_record_offset":25404,"ancestor_record_offset":24680,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":24944,"parent_record_offset":24912,"ancestor_record_offset":24680,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":24680,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":25104,"parent_record_offset":25072,"ancestor_record_offset":24680,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":25200,"parent_record_offset":25104,"ancestor_record_offset":24680,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":25072,"parent_record_offset":25048,"ancestor_record_offset":24680,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":24680,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":395,"record_offset":25224,"parent_record_offset":25200,"ancestor_record_offset":24680,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":25504,"parent_record_offset":25472,"ancestor_record_offset":24680,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":25788,"parent_record_offset":25504,"ancestor_record_offset":24680,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":25760,"parent_record_offset":25504,"ancestor_record_offset":24680,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":395,"record_offset":25564,"parent_record_offset":25504,"ancestor_record_offset":24680,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":395,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":24680,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":395,"record_offset":31408,"parent_record_offset":31384,"ancestor_record_offset":24680,"inlinee":24536,"source_line":436,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":28724,"parent_record_offset":28696,"ancestor_record_offset":24680,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":28756,"parent_record_offset":28724,"ancestor_record_offset":24680,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":395,"record_offset":28696,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":28556,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":29168,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":395,"record_offset":28640,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":395,"record_offset":28528,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":395,"record_offset":29048,"parent_record_offset":29024,"ancestor_record_offset":24680,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":30384,"parent_record_offset":30352,"ancestor_record_offset":24680,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":28312,"parent_record_offset":28288,"ancestor_record_offset":24680,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":28336,"parent_record_offset":28312,"ancestor_record_offset":24680,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":395,"record_offset":29024,"parent_record_offset":28856,"ancestor_record_offset":24680,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":395,"record_offset":28368,"parent_record_offset":28336,"ancestor_record_offset":24680,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":28964,"parent_record_offset":28880,"ancestor_record_offset":24680,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":395,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":24680,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28908,"parent_record_offset":28880,"ancestor_record_offset":24680,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":29260,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28880,"parent_record_offset":28856,"ancestor_record_offset":24680,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28420,"parent_record_offset":28396,"ancestor_record_offset":24680,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28396,"parent_record_offset":28336,"ancestor_record_offset":24680,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28444,"parent_record_offset":28420,"ancestor_record_offset":24680,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28500,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":395,"record_offset":28856,"parent_record_offset":28444,"ancestor_record_offset":24680,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":26312,"parent_record_offset":26280,"ancestor_record_offset":24680,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":31536,"parent_record_offset":31504,"ancestor_record_offset":24680,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":32036,"parent_record_offset":31776,"ancestor_record_offset":24680,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":395,"record_offset":26444,"parent_record_offset":26416,"ancestor_record_offset":24680,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":26520,"parent_record_offset":26376,"ancestor_record_offset":24680,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":395,"record_offset":26780,"parent_record_offset":26520,"ancestor_record_offset":24680,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":395,"record_offset":26472,"parent_record_offset":26444,"ancestor_record_offset":24680,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":395,"record_offset":26376,"parent_record_offset":26344,"ancestor_record_offset":24680,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":395,"record_offset":26344,"parent_record_offset":26312,"ancestor_record_offset":24680,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::create_plaster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":31384,"parent_record_offset":31360,"ancestor_record_offset":24680,"inlinee":24538,"source_line":227,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDA
// 首条位置: {"module":395,"inlinee":24538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":395,"record_offset":31408,"parent_record_offset":31384,"ancestor_record_offset":31384,"inlinee":24536,"source_line":436,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1443,"record_offset":8196,"rva":19032496,"va":5387741616}
// 编译实例 RVAs: 0x12269b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18790909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e80346e1fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5387744632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12660826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8a6cf3eff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387742309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12668682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8f6b03eff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387750181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14082bd60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10473291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8b53060ff","size":5,"soft_fail":false,"target_va":5377277280,"va":5387750566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10443345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8afa560ff","size":5,"soft_fail":false,"target_va":5377307216,"va":5387750556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9039334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e81a1276ff","size":5,"soft_fail":false,"target_va":5378702496,"va":5387741825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6571827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8cdb89bff","size":5,"soft_fail":false,"target_va":5381170352,"va":5387742174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5810563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e87d56a7ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387745086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fc890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-181777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8ef39fdff","size":5,"soft_fail":false,"target_va":5387569296,"va":5387751068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141207c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e82f06feff","size":5,"soft_fail":false,"target_va":5387615248,"va":5387744732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdca60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14370124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e84c45db00","size":5,"soft_fail":false,"target_va":5402118752,"va":5387748623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24101598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8dec26f01","size":5,"soft_fail":false,"target_va":5411847776,"va":5387746173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ef0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24101193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e849c16f01","size":5,"soft_fail":false,"target_va":5411847920,"va":5387746722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23788777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8e9fc6a01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387752210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13392653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8f3a433ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5387748792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23597243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8bb106801","size":5,"soft_fail":false,"target_va":5411339744,"va":5387742496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23789057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e801fe6a01","size":5,"soft_fail":false,"target_va":5411540963,"va":5387751901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-104642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e83e67feff","size":5,"soft_fail":false,"target_va":5387645728,"va":5387750365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5326413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8b3b9aeff","size":5,"soft_fail":false,"target_va":5382424224,"va":5387750632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23790154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e84a026b01","size":5,"soft_fail":false,"target_va":5411541584,"va":5387751425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23790819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8e3046b01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387750952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23790822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8e6046b01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387751237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t23790907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e83b056b01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387751384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23790361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e819036b01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387751538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e824e6ffff","size":5,"soft_fail":false,"target_va":5387741440,"va":5387748055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23791486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e87e076b01","size":5,"soft_fail":false,"target_va":5411543088,"va":5387751597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23791176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e848066b01","size":5,"soft_fail":false,"target_va":5411543360,"va":5387752179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9714487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8c9c46bff","size":5,"soft_fail":false,"target_va":5378028416,"va":5387742898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9714485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8cbc46bff","size":5,"soft_fail":false,"target_va":5378028432,"va":5387742912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7948292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8fcb786ff","size":5,"soft_fail":false,"target_va":5379793584,"va":5387741871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6358827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8d5f89eff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387744358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6353600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8400d9fff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387744235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6361076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e80cf09eff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387747551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6359280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e810f79eff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387743979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::device_address
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6633833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e897c69aff","size":5,"soft_fail":false,"target_va":5381114880,"va":5387748708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_only
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6633150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e842c99aff","size":5,"soft_fail":false,"target_va":5381115424,"va":5387748569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23896489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8a9a16c01","size":5,"soft_fail":false,"target_va":5411640768,"va":5387744274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-191153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e84f15fdff","size":5,"soft_fail":false,"target_va":5387564432,"va":5387755580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23785023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e83fee6a01","size":5,"soft_fail":false,"target_va":5411537104,"va":5387752076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::plaster_systems::split_wall_into_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11309715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e86d6d53ff","size":5,"soft_fail":false,"target_va":5376433248,"va":5387742958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t103390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8de930100","size":5,"soft_fail":false,"target_va":5387845824,"va":5387742429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::half_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t103424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e800940100","size":5,"soft_fail":false,"target_va":5387845872,"va":5387742443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5832500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8cc00a7ff","size":5,"soft_fail":false,"target_va":5381912800,"va":5387745295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5822736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8f026a7ff","size":5,"soft_fail":false,"target_va":5381922320,"va":5387745051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5858062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e8f29ca6ff","size":5,"soft_fail":false,"target_va":5381890192,"va":5387748249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5854128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387741616,"raw_hex":"e850aca6ff","size":5,"soft_fail":false,"target_va":5381891536,"va":5387745659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":34248,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1443,"record_offset":34400,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":8820,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":34608,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1443,"record_offset":34484,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1443,"record_offset":35480,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1443,"record_offset":34824,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1443,"record_offset":34796,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":8876,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":35164,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1443,"record_offset":35540,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::generate_plaster_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":10480,"parent_record_offset":8196,"ancestor_record_offset":8196,"inlinee":70968,"source_line":316,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":8196,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":8196,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":8196,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12616,"parent_record_offset":12440,"ancestor_record_offset":8196,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13836,"parent_record_offset":13744,"ancestor_record_offset":8196,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1443,"record_offset":35856,"parent_record_offset":35832,"ancestor_record_offset":8196,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9204,"parent_record_offset":9180,"ancestor_record_offset":8196,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":8196,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":8196,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":25740,"parent_record_offset":25716,"ancestor_record_offset":8196,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":8196,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":8196,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":25716,"parent_record_offset":25684,"ancestor_record_offset":8196,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":8196,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":8196,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":8196,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12440,"parent_record_offset":12408,"ancestor_record_offset":8196,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":8196,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":8196,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12476,"parent_record_offset":12440,"ancestor_record_offset":8196,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1443,"record_offset":34424,"parent_record_offset":34400,"ancestor_record_offset":8196,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":8196,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11508,"parent_record_offset":11468,"ancestor_record_offset":8196,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":8196,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9156,"parent_record_offset":9056,"ancestor_record_offset":8196,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11440,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":8196,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":34448,"parent_record_offset":34424,"ancestor_record_offset":8196,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":14536,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":26572,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13168,"parent_record_offset":12768,"ancestor_record_offset":8196,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13380,"parent_record_offset":12768,"ancestor_record_offset":8196,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12316,"parent_record_offset":12032,"ancestor_record_offset":8196,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12348,"parent_record_offset":12316,"ancestor_record_offset":8196,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":19536,"parent_record_offset":19504,"ancestor_record_offset":8196,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":27728,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12720,"parent_record_offset":12032,"ancestor_record_offset":8196,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":36520,"parent_record_offset":36496,"ancestor_record_offset":8196,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":8196,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1443,"record_offset":35188,"parent_record_offset":35164,"ancestor_record_offset":8196,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1443,"record_offset":35236,"parent_record_offset":35212,"ancestor_record_offset":8196,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1443,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":8196,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1443,"record_offset":9356,"parent_record_offset":9024,"ancestor_record_offset":8196,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1443,"record_offset":35508,"parent_record_offset":35480,"ancestor_record_offset":8196,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1443,"record_offset":9024,"parent_record_offset":8976,"ancestor_record_offset":8196,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10348,"parent_record_offset":10324,"ancestor_record_offset":8196,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":9828,"parent_record_offset":8976,"ancestor_record_offset":8196,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":8196,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10324,"parent_record_offset":8976,"ancestor_record_offset":8196,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":8196,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1443,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":8196,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8196,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":8196,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":8196,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":8196,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":8196,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9568,"parent_record_offset":8976,"ancestor_record_offset":8196,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1443,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":8196,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":8196,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1443,"record_offset":34912,"parent_record_offset":34884,"ancestor_record_offset":8196,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1443,"record_offset":34944,"parent_record_offset":34912,"ancestor_record_offset":8196,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1443,"record_offset":34856,"parent_record_offset":34824,"ancestor_record_offset":8196,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1443,"record_offset":34884,"parent_record_offset":34856,"ancestor_record_offset":8196,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1443,"record_offset":8944,"parent_record_offset":8912,"ancestor_record_offset":8196,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1443,"record_offset":8976,"parent_record_offset":8944,"ancestor_record_offset":8196,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1443,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":8196,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":8196,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":8196,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1443,"record_offset":25360,"parent_record_offset":25328,"ancestor_record_offset":8196,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1443,"record_offset":25476,"parent_record_offset":25384,"ancestor_record_offset":8196,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1443,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":8196,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17464,"parent_record_offset":17436,"ancestor_record_offset":8196,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":8196,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":8196,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17404,"parent_record_offset":17372,"ancestor_record_offset":8196,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17436,"parent_record_offset":17404,"ancestor_record_offset":8196,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17608,"parent_record_offset":17436,"ancestor_record_offset":8196,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":8196,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":21400,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":21428,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":8196,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":8196,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":19376,"parent_record_offset":19352,"ancestor_record_offset":8196,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":8196,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":28204,"parent_record_offset":28164,"ancestor_record_offset":8196,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":20692,"parent_record_offset":20516,"ancestor_record_offset":8196,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":15188,"parent_record_offset":15156,"ancestor_record_offset":8196,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":8196,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":26096,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":19416,"parent_record_offset":19264,"ancestor_record_offset":8196,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":18544,"parent_record_offset":18320,"ancestor_record_offset":8196,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":20328,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1443,"record_offset":19444,"parent_record_offset":19264,"ancestor_record_offset":8196,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1443,"record_offset":15624,"parent_record_offset":15568,"ancestor_record_offset":8196,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1443,"record_offset":25328,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":8196,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":8196,"inlinee":41527,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":14236,"parent_record_offset":14200,"ancestor_record_offset":8196,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":8196,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12148,"parent_record_offset":12032,"ancestor_record_offset":8196,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":14364,"parent_record_offset":14272,"ancestor_record_offset":8196,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":8196,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":28136,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":20580,"parent_record_offset":20516,"ancestor_record_offset":8196,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":26936,"parent_record_offset":26880,"ancestor_record_offset":8196,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":8196,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13420,"parent_record_offset":12032,"ancestor_record_offset":8196,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1443,"record_offset":14200,"parent_record_offset":14164,"ancestor_record_offset":8196,"inlinee":9239,"source_line":49,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1443,"record_offset":14448,"parent_record_offset":14364,"ancestor_record_offset":8196,"inlinee":9245,"source_line":53,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14568,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14600,"parent_record_offset":14568,"ancestor_record_offset":8196,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":15156,"parent_record_offset":15088,"ancestor_record_offset":8196,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14632,"parent_record_offset":14600,"ancestor_record_offset":8196,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14164,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":30608,"parent_record_offset":30584,"ancestor_record_offset":8196,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1443,"record_offset":30724,"parent_record_offset":30696,"ancestor_record_offset":8196,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1443,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":8196,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1443,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":8196,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":8196,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":8196,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":14492,"parent_record_offset":14272,"ancestor_record_offset":8196,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":13804,"parent_record_offset":13772,"ancestor_record_offset":8196,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":8196,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":8196,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":8196,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1443,"record_offset":16940,"parent_record_offset":16904,"ancestor_record_offset":8196,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1443,"record_offset":9300,"parent_record_offset":9024,"ancestor_record_offset":8196,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18288,"parent_record_offset":18180,"ancestor_record_offset":8196,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18232,"parent_record_offset":18180,"ancestor_record_offset":8196,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":28516,"parent_record_offset":28488,"ancestor_record_offset":8196,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":8196,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18204,"parent_record_offset":18180,"ancestor_record_offset":8196,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":10044,"parent_record_offset":9980,"ancestor_record_offset":8196,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":8196,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1443,"record_offset":25420,"parent_record_offset":25384,"ancestor_record_offset":8196,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1443,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":8196,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":8196,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":19304,"parent_record_offset":19264,"ancestor_record_offset":8196,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":28236,"parent_record_offset":28164,"ancestor_record_offset":8196,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":8196,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":10108,"parent_record_offset":9928,"ancestor_record_offset":8196,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":11928,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":15308,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":17024,"parent_record_offset":16996,"ancestor_record_offset":8196,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":17340,"parent_record_offset":17200,"ancestor_record_offset":8196,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":25524,"parent_record_offset":25500,"ancestor_record_offset":8196,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":8196,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=77
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":11648,"parent_record_offset":11468,"ancestor_record_offset":8196,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":9520,"parent_record_offset":9488,"ancestor_record_offset":8196,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":8196,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":8196,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":16508,"parent_record_offset":16484,"ancestor_record_offset":8196,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":34548,"parent_record_offset":34484,"ancestor_record_offset":8196,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":8196,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":8196,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12948,"parent_record_offset":12912,"ancestor_record_offset":8196,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":25548,"parent_record_offset":25524,"ancestor_record_offset":8196,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":34356,"parent_record_offset":34328,"ancestor_record_offset":8196,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13576,"parent_record_offset":13516,"ancestor_record_offset":8196,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1443,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":8196,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1443,"record_offset":27660,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":8196,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":8196,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27876,"parent_record_offset":27852,"ancestor_record_offset":8196,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":18668,"parent_record_offset":18632,"ancestor_record_offset":8196,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27760,"parent_record_offset":27728,"ancestor_record_offset":8196,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":9096,"parent_record_offset":9056,"ancestor_record_offset":8196,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":18696,"parent_record_offset":18668,"ancestor_record_offset":8196,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27852,"parent_record_offset":27828,"ancestor_record_offset":8196,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27828,"parent_record_offset":27792,"ancestor_record_offset":8196,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27792,"parent_record_offset":27760,"ancestor_record_offset":8196,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":8196,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":8196,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":8196,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":8196,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":8196,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":8196,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":14272,"parent_record_offset":14236,"ancestor_record_offset":8196,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12912,"parent_record_offset":12884,"ancestor_record_offset":8196,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":8196,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16412,"parent_record_offset":16296,"ancestor_record_offset":8196,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12976,"parent_record_offset":12912,"ancestor_record_offset":8196,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":27952,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":28020,"parent_record_offset":27952,"ancestor_record_offset":8196,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1443,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":8196,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1443,"record_offset":16124,"parent_record_offset":16088,"ancestor_record_offset":8196,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1443,"record_offset":16264,"parent_record_offset":16232,"ancestor_record_offset":8196,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1443,"record_offset":16232,"parent_record_offset":16196,"ancestor_record_offset":8196,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16172,"parent_record_offset":16148,"ancestor_record_offset":8196,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":8196,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16148,"parent_record_offset":16124,"ancestor_record_offset":8196,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16736,"parent_record_offset":16196,"ancestor_record_offset":8196,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16876,"parent_record_offset":16844,"ancestor_record_offset":8196,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16972,"parent_record_offset":16876,"ancestor_record_offset":8196,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16844,"parent_record_offset":16816,"ancestor_record_offset":8196,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16816,"parent_record_offset":16088,"ancestor_record_offset":8196,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":8196,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16296,"parent_record_offset":16264,"ancestor_record_offset":8196,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16580,"parent_record_offset":16296,"ancestor_record_offset":8196,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16552,"parent_record_offset":16296,"ancestor_record_offset":8196,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16356,"parent_record_offset":16296,"ancestor_record_offset":8196,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1443,"record_offset":13920,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1443,"record_offset":11852,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9388,"parent_record_offset":8976,"ancestor_record_offset":8196,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":8196,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":8196,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":8196,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1443,"record_offset":22080,"parent_record_offset":22056,"ancestor_record_offset":8196,"inlinee":13667,"source_line":146,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1443,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":8196,"inlinee":32524,"source_line":202,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":22056,"parent_record_offset":21736,"ancestor_record_offset":8196,"inlinee":24543,"source_line":320,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21148,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":29553,"source_line":977,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21064,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":32523,"source_line":618,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20104,"parent_record_offset":20068,"ancestor_record_offset":8196,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20068,"parent_record_offset":20028,"ancestor_record_offset":8196,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":8196,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21772,"parent_record_offset":21736,"ancestor_record_offset":8196,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20004,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":19976,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21008,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20840,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20896,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":19989,"source_line":1333,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20728,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20784,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19836,"parent_record_offset":19804,"ancestor_record_offset":8196,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":8196,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":8196,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":29200,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":8196,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":25784,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":28664,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":13663,"source_line":1280,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":26480,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":28488,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19632,"parent_record_offset":19504,"ancestor_record_offset":8196,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1443,"record_offset":18180,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1443,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":8196,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17076,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17116,"parent_record_offset":17076,"ancestor_record_offset":8196,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":22312,"parent_record_offset":22272,"ancestor_record_offset":8196,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":22880,"parent_record_offset":22608,"ancestor_record_offset":8196,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1443,"record_offset":17268,"parent_record_offset":17240,"ancestor_record_offset":8196,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17372,"parent_record_offset":17200,"ancestor_record_offset":8196,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17644,"parent_record_offset":17372,"ancestor_record_offset":8196,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":16088,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15976,"parent_record_offset":15932,"ancestor_record_offset":8196,"inlinee":48744,"source_line":1716,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15804,"parent_record_offset":15772,"ancestor_record_offset":8196,"inlinee":11444,"source_line":952,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15896,"parent_record_offset":15772,"ancestor_record_offset":8196,"inlinee":35796,"source_line":919,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14124,"parent_record_offset":14096,"ancestor_record_offset":8196,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14096,"parent_record_offset":14068,"ancestor_record_offset":8196,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":33720,"parent_record_offset":33664,"ancestor_record_offset":8196,"inlinee":6214,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14068,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4601,"source_line":1846,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":30696,"parent_record_offset":30672,"ancestor_record_offset":8196,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":8196,"inlinee":43484,"source_line":2228,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15932,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15772,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":35795,"source_line":2290,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":8196,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":21676,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4581,"source_line":214,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":19868,"parent_record_offset":19836,"ancestor_record_offset":8196,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1443,"record_offset":17296,"parent_record_offset":17268,"ancestor_record_offset":8196,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1443,"record_offset":17200,"parent_record_offset":17156,"ancestor_record_offset":8196,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1443,"record_offset":17156,"parent_record_offset":17116,"ancestor_record_offset":8196,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":11980,"parent_record_offset":11928,"ancestor_record_offset":8196,"inlinee":70969,"source_line":331,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":8196,"inlinee":70972,"source_line":339,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":8196,"inlinee":70973,"source_line":340,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14404,"parent_record_offset":14364,"ancestor_record_offset":8196,"inlinee":70831,"source_line":363,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14864,"parent_record_offset":14840,"ancestor_record_offset":8196,"inlinee":70832,"source_line":379,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":8912,"parent_record_offset":8876,"ancestor_record_offset":8196,"inlinee":70967,"source_line":263,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":12032,"parent_record_offset":11980,"ancestor_record_offset":8196,"inlinee":70970,"source_line":792,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":8196,"inlinee":70771,"source_line":799,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::sd_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":21736,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":24547,"source_line":964,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13516,"parent_record_offset":12032,"ancestor_record_offset":8196,"inlinee":70971,"source_line":972,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18320,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":19476,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":24690,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":19504,"parent_record_offset":19476,"ancestor_record_offset":8196,"inlinee":24691,"source_line":812,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":11900,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18732,"parent_record_offset":18632,"ancestor_record_offset":8196,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18600,"parent_record_offset":18320,"ancestor_record_offset":8196,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18572,"parent_record_offset":18320,"ancestor_record_offset":8196,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18368,"parent_record_offset":18320,"ancestor_record_offset":8196,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18404,"parent_record_offset":18368,"ancestor_record_offset":8196,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18508,"parent_record_offset":18480,"ancestor_record_offset":8196,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18480,"parent_record_offset":18320,"ancestor_record_offset":8196,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1443,"record_offset":19264,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18632,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1443,"record_offset":19724,"parent_record_offset":19476,"ancestor_record_offset":8196,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1443,"record_offset":26128,"parent_record_offset":10480,"ancestor_record_offset":8196,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11537
// 首条位置: {"module":1443,"inlinee":70967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1443,"record_offset":8944,"parent_record_offset":8912,"ancestor_record_offset":8912,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9204,"parent_record_offset":9180,"ancestor_record_offset":8912,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":8912,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":8912,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":8912,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9156,"parent_record_offset":9056,"ancestor_record_offset":8912,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":8912,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1443,"record_offset":9356,"parent_record_offset":9024,"ancestor_record_offset":8912,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1443,"record_offset":9024,"parent_record_offset":8976,"ancestor_record_offset":8912,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10348,"parent_record_offset":10324,"ancestor_record_offset":8912,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":9828,"parent_record_offset":8976,"ancestor_record_offset":8912,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":8912,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10324,"parent_record_offset":8976,"ancestor_record_offset":8912,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1443,"record_offset":10372,"parent_record_offset":10348,"ancestor_record_offset":8912,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1443,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":8912,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":8912,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":9928,"parent_record_offset":9904,"ancestor_record_offset":8912,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1443,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":8912,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":8912,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":8912,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":9568,"parent_record_offset":8976,"ancestor_record_offset":8912,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1443,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":8912,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1443,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":8912,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1443,"record_offset":8976,"parent_record_offset":8944,"ancestor_record_offset":8912,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1443,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":8912,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1443,"record_offset":9416,"parent_record_offset":9388,"ancestor_record_offset":8912,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":8912,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1443,"record_offset":9300,"parent_record_offset":9024,"ancestor_record_offset":8912,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":10044,"parent_record_offset":9980,"ancestor_record_offset":8912,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":8912,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":10108,"parent_record_offset":9928,"ancestor_record_offset":8912,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":9520,"parent_record_offset":9488,"ancestor_record_offset":8912,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1443,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":8912,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":9124,"parent_record_offset":9096,"ancestor_record_offset":8912,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":9096,"parent_record_offset":9056,"ancestor_record_offset":8912,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9388,"parent_record_offset":8976,"ancestor_record_offset":8912,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":8912,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":8912,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1443,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":8912,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1443,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":8912,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11538
// 首条位置: {"module":1443,"inlinee":70968}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11440,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":14536,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":26572,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":25628,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":15344,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":27084,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":27728,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":21400,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":21428,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":26096,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":29264,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":20328,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1443,"record_offset":25328,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":28136,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":18084,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14568,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":19409,"source_line":3107,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14164,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":13952,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":11928,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":15308,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":22232,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":20216,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1443,"record_offset":27660,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":20384,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":27952,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1443,"record_offset":13920,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1443,"record_offset":11852,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21148,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":29553,"source_line":977,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21064,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":32523,"source_line":618,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20004,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":19976,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21008,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20840,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20896,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":19989,"source_line":1333,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20728,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$56::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20784,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":10959,"source_line":1468,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":29200,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":25784,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":28664,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":13663,"source_line":1280,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":26480,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":28488,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":28752,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1443,"record_offset":18180,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17076,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":16088,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14068,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4601,"source_line":1846,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15932,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15772,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":35795,"source_line":2290,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":21676,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4581,"source_line":214,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":28108,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::sd_box
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":21736,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":24547,"source_line":964,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":25960,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":70971,"source_line":972,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18320,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":19476,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":24690,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":11900,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1443,"record_offset":19264,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18632,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1443,"record_offset":26128,"parent_record_offset":10480,"ancestor_record_offset":10480,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":10480,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":10480,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":10480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12616,"parent_record_offset":12440,"ancestor_record_offset":10480,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13836,"parent_record_offset":13744,"ancestor_record_offset":10480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":10480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":10480,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":10480,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":25740,"parent_record_offset":25716,"ancestor_record_offset":10480,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":10480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":10480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":25716,"parent_record_offset":25684,"ancestor_record_offset":10480,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":10480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":10480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":10480,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12440,"parent_record_offset":12408,"ancestor_record_offset":10480,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":10480,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":10480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12476,"parent_record_offset":12440,"ancestor_record_offset":10480,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":10480,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":11508,"parent_record_offset":11468,"ancestor_record_offset":10480,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":10480,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13168,"parent_record_offset":12768,"ancestor_record_offset":10480,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13380,"parent_record_offset":12768,"ancestor_record_offset":10480,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12316,"parent_record_offset":12032,"ancestor_record_offset":10480,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12348,"parent_record_offset":12316,"ancestor_record_offset":10480,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":19536,"parent_record_offset":19504,"ancestor_record_offset":10480,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12720,"parent_record_offset":12032,"ancestor_record_offset":10480,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1443,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":10480,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":10480,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":10480,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1443,"record_offset":25360,"parent_record_offset":25328,"ancestor_record_offset":10480,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1443,"record_offset":25476,"parent_record_offset":25384,"ancestor_record_offset":10480,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17464,"parent_record_offset":17436,"ancestor_record_offset":10480,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17516,"parent_record_offset":17492,"ancestor_record_offset":10480,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":10480,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17404,"parent_record_offset":17372,"ancestor_record_offset":10480,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17436,"parent_record_offset":17404,"ancestor_record_offset":10480,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17608,"parent_record_offset":17436,"ancestor_record_offset":10480,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1443,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":10480,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":10480,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":19376,"parent_record_offset":19352,"ancestor_record_offset":10480,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":10480,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":28204,"parent_record_offset":28164,"ancestor_record_offset":10480,"inlinee":5290,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":20692,"parent_record_offset":20516,"ancestor_record_offset":10480,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":15188,"parent_record_offset":15156,"ancestor_record_offset":10480,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":10480,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":19416,"parent_record_offset":19264,"ancestor_record_offset":10480,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":18544,"parent_record_offset":18320,"ancestor_record_offset":10480,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":22024,"parent_record_offset":21736,"ancestor_record_offset":10480,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1443,"record_offset":19444,"parent_record_offset":19264,"ancestor_record_offset":10480,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1443,"record_offset":15624,"parent_record_offset":15568,"ancestor_record_offset":10480,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":10480,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":10480,"inlinee":41527,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":14236,"parent_record_offset":14200,"ancestor_record_offset":10480,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":10480,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12148,"parent_record_offset":12032,"ancestor_record_offset":10480,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":14364,"parent_record_offset":14272,"ancestor_record_offset":10480,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":10480,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":20580,"parent_record_offset":20516,"ancestor_record_offset":10480,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":26936,"parent_record_offset":26880,"ancestor_record_offset":10480,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":10480,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13420,"parent_record_offset":12032,"ancestor_record_offset":10480,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1443,"record_offset":14200,"parent_record_offset":14164,"ancestor_record_offset":10480,"inlinee":9239,"source_line":49,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1443,"record_offset":14448,"parent_record_offset":14364,"ancestor_record_offset":10480,"inlinee":9245,"source_line":53,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14600,"parent_record_offset":14568,"ancestor_record_offset":10480,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":15156,"parent_record_offset":15088,"ancestor_record_offset":10480,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":14632,"parent_record_offset":14600,"ancestor_record_offset":10480,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1443,"record_offset":30608,"parent_record_offset":30584,"ancestor_record_offset":10480,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1443,"record_offset":30724,"parent_record_offset":30696,"ancestor_record_offset":10480,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1443,"record_offset":25500,"parent_record_offset":25476,"ancestor_record_offset":10480,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1443,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":10480,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":10480,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":10480,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":14492,"parent_record_offset":14272,"ancestor_record_offset":10480,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":13804,"parent_record_offset":13772,"ancestor_record_offset":10480,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20604,"parent_record_offset":20580,"ancestor_record_offset":10480,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":10480,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":1443,"record_offset":20628,"parent_record_offset":20604,"ancestor_record_offset":10480,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1443,"record_offset":16940,"parent_record_offset":16904,"ancestor_record_offset":10480,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18288,"parent_record_offset":18180,"ancestor_record_offset":10480,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18232,"parent_record_offset":18180,"ancestor_record_offset":10480,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":28516,"parent_record_offset":28488,"ancestor_record_offset":10480,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":10480,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":18204,"parent_record_offset":18180,"ancestor_record_offset":10480,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":17240,"parent_record_offset":17200,"ancestor_record_offset":10480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":10480,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1443,"record_offset":25420,"parent_record_offset":25384,"ancestor_record_offset":10480,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1443,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":10480,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":10480,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":19304,"parent_record_offset":19264,"ancestor_record_offset":10480,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1443,"record_offset":28236,"parent_record_offset":28164,"ancestor_record_offset":10480,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":13076,"parent_record_offset":12812,"ancestor_record_offset":10480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":18444,"parent_record_offset":18404,"ancestor_record_offset":10480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":17024,"parent_record_offset":16996,"ancestor_record_offset":10480,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":17340,"parent_record_offset":17200,"ancestor_record_offset":10480,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":25524,"parent_record_offset":25500,"ancestor_record_offset":10480,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":10480,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":11648,"parent_record_offset":11468,"ancestor_record_offset":10480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":13548,"parent_record_offset":13516,"ancestor_record_offset":10480,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":10480,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":10480,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":16508,"parent_record_offset":16484,"ancestor_record_offset":10480,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":16484,"parent_record_offset":16460,"ancestor_record_offset":10480,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":10480,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12948,"parent_record_offset":12912,"ancestor_record_offset":10480,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":25548,"parent_record_offset":25524,"ancestor_record_offset":10480,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13576,"parent_record_offset":13516,"ancestor_record_offset":10480,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27900,"parent_record_offset":27876,"ancestor_record_offset":10480,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27876,"parent_record_offset":27852,"ancestor_record_offset":10480,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":18668,"parent_record_offset":18632,"ancestor_record_offset":10480,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27760,"parent_record_offset":27728,"ancestor_record_offset":10480,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":18696,"parent_record_offset":18668,"ancestor_record_offset":10480,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27852,"parent_record_offset":27828,"ancestor_record_offset":10480,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27828,"parent_record_offset":27792,"ancestor_record_offset":10480,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1443,"record_offset":27792,"parent_record_offset":27760,"ancestor_record_offset":10480,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":10480,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":10480,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":10480,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":10480,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":10480,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":10480,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":14272,"parent_record_offset":14236,"ancestor_record_offset":10480,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12912,"parent_record_offset":12884,"ancestor_record_offset":10480,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":10480,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":16412,"parent_record_offset":16296,"ancestor_record_offset":10480,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12976,"parent_record_offset":12912,"ancestor_record_offset":10480,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":28020,"parent_record_offset":27952,"ancestor_record_offset":10480,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1443,"record_offset":16904,"parent_record_offset":16876,"ancestor_record_offset":10480,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1443,"record_offset":16124,"parent_record_offset":16088,"ancestor_record_offset":10480,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1443,"record_offset":16264,"parent_record_offset":16232,"ancestor_record_offset":10480,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1443,"record_offset":16232,"parent_record_offset":16196,"ancestor_record_offset":10480,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16172,"parent_record_offset":16148,"ancestor_record_offset":10480,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":10480,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16148,"parent_record_offset":16124,"ancestor_record_offset":10480,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1443,"record_offset":16736,"parent_record_offset":16196,"ancestor_record_offset":10480,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16876,"parent_record_offset":16844,"ancestor_record_offset":10480,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16972,"parent_record_offset":16876,"ancestor_record_offset":10480,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16844,"parent_record_offset":16816,"ancestor_record_offset":10480,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16816,"parent_record_offset":16088,"ancestor_record_offset":10480,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1443,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":10480,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16296,"parent_record_offset":16264,"ancestor_record_offset":10480,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16580,"parent_record_offset":16296,"ancestor_record_offset":10480,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16552,"parent_record_offset":16296,"ancestor_record_offset":10480,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1443,"record_offset":16356,"parent_record_offset":16296,"ancestor_record_offset":10480,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1443,"record_offset":22080,"parent_record_offset":22056,"ancestor_record_offset":10480,"inlinee":13667,"source_line":146,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1443,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":10480,"inlinee":32524,"source_line":202,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":22056,"parent_record_offset":21736,"ancestor_record_offset":10480,"inlinee":24543,"source_line":320,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20104,"parent_record_offset":20068,"ancestor_record_offset":10480,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20068,"parent_record_offset":20028,"ancestor_record_offset":10480,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":20028,"parent_record_offset":20004,"ancestor_record_offset":10480,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21772,"parent_record_offset":21736,"ancestor_record_offset":10480,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":22140,"parent_record_offset":21736,"ancestor_record_offset":10480,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19836,"parent_record_offset":19804,"ancestor_record_offset":10480,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19804,"parent_record_offset":19772,"ancestor_record_offset":10480,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19772,"parent_record_offset":19748,"ancestor_record_offset":10480,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":10480,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1443,"record_offset":19632,"parent_record_offset":19504,"ancestor_record_offset":10480,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17116,"parent_record_offset":17076,"ancestor_record_offset":10480,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":22312,"parent_record_offset":22272,"ancestor_record_offset":10480,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":22880,"parent_record_offset":22608,"ancestor_record_offset":10480,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1443,"record_offset":17268,"parent_record_offset":17240,"ancestor_record_offset":10480,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17372,"parent_record_offset":17200,"ancestor_record_offset":10480,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1443,"record_offset":17644,"parent_record_offset":17372,"ancestor_record_offset":10480,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15976,"parent_record_offset":15932,"ancestor_record_offset":10480,"inlinee":48744,"source_line":1716,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15804,"parent_record_offset":15772,"ancestor_record_offset":10480,"inlinee":11444,"source_line":952,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":15896,"parent_record_offset":15772,"ancestor_record_offset":10480,"inlinee":35796,"source_line":919,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14124,"parent_record_offset":14096,"ancestor_record_offset":10480,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":14096,"parent_record_offset":14068,"ancestor_record_offset":10480,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":33720,"parent_record_offset":33664,"ancestor_record_offset":10480,"inlinee":6214,"source_line":659,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":30696,"parent_record_offset":30672,"ancestor_record_offset":10480,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":10480,"inlinee":43484,"source_line":2228,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1443,"record_offset":30632,"parent_record_offset":30608,"ancestor_record_offset":10480,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":21112,"parent_record_offset":21088,"ancestor_record_offset":10480,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":19868,"parent_record_offset":19836,"ancestor_record_offset":10480,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1443,"record_offset":17296,"parent_record_offset":17268,"ancestor_record_offset":10480,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1443,"record_offset":17200,"parent_record_offset":17156,"ancestor_record_offset":10480,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1443,"record_offset":17156,"parent_record_offset":17116,"ancestor_record_offset":10480,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":11980,"parent_record_offset":11928,"ancestor_record_offset":10480,"inlinee":70969,"source_line":331,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13980,"parent_record_offset":13952,"ancestor_record_offset":10480,"inlinee":70972,"source_line":339,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":10480,"inlinee":70973,"source_line":340,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14404,"parent_record_offset":14364,"ancestor_record_offset":10480,"inlinee":70831,"source_line":363,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":14864,"parent_record_offset":14840,"ancestor_record_offset":10480,"inlinee":70832,"source_line":379,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":12032,"parent_record_offset":11980,"ancestor_record_offset":10480,"inlinee":70970,"source_line":792,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":10480,"inlinee":70771,"source_line":799,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13516,"parent_record_offset":12032,"ancestor_record_offset":10480,"inlinee":70971,"source_line":972,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":19504,"parent_record_offset":19476,"ancestor_record_offset":10480,"inlinee":24691,"source_line":812,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1443,"record_offset":18732,"parent_record_offset":18632,"ancestor_record_offset":10480,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18600,"parent_record_offset":18320,"ancestor_record_offset":10480,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18572,"parent_record_offset":18320,"ancestor_record_offset":10480,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18368,"parent_record_offset":18320,"ancestor_record_offset":10480,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18404,"parent_record_offset":18368,"ancestor_record_offset":10480,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18508,"parent_record_offset":18480,"ancestor_record_offset":10480,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1443,"record_offset":18480,"parent_record_offset":18320,"ancestor_record_offset":10480,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1443,"record_offset":19724,"parent_record_offset":19476,"ancestor_record_offset":10480,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 331
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11539
// 首条位置: {"module":1443,"inlinee":70969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":12032,"parent_record_offset":11980,"ancestor_record_offset":11980,"inlinee":70970,"source_line":792,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":11980,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":11980,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":11980,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12616,"parent_record_offset":12440,"ancestor_record_offset":11980,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13836,"parent_record_offset":13744,"ancestor_record_offset":11980,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13292,"parent_record_offset":13252,"ancestor_record_offset":11980,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":11980,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":11980,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":11980,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":11980,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":11980,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":11980,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":11980,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12440,"parent_record_offset":12408,"ancestor_record_offset":11980,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":11980,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":11980,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12476,"parent_record_offset":12440,"ancestor_record_offset":11980,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13252,"parent_record_offset":12768,"ancestor_record_offset":11980,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":11980,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13168,"parent_record_offset":12768,"ancestor_record_offset":11980,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13380,"parent_record_offset":12768,"ancestor_record_offset":11980,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12316,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12348,"parent_record_offset":12316,"ancestor_record_offset":11980,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12720,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1443,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":11980,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":11980,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":11980,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":11980,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":11980,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":11980,"inlinee":41527,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":11980,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12148,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":11980,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13420,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":11980,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":11980,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":13804,"parent_record_offset":13772,"ancestor_record_offset":11980,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":11980,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":13076,"parent_record_offset":12812,"ancestor_record_offset":11980,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":11980,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13672,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":13548,"parent_record_offset":13516,"ancestor_record_offset":11980,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":11980,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":11980,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11980,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12948,"parent_record_offset":12912,"ancestor_record_offset":11980,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13576,"parent_record_offset":13516,"ancestor_record_offset":11980,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":11980,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":11980,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12912,"parent_record_offset":12884,"ancestor_record_offset":11980,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":11980,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12976,"parent_record_offset":12912,"ancestor_record_offset":11980,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":11980,"inlinee":70771,"source_line":799,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13516,"parent_record_offset":12032,"ancestor_record_offset":11980,"inlinee":70971,"source_line":972,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 339
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1153C
// 首条位置: {"module":1443,"inlinee":70972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1153D
// 首条位置: {"module":1443,"inlinee":70973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114AF
// 首条位置: {"module":1417,"inlinee":70831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 379
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114B0
// 首条位置: {"module":1417,"inlinee":70832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 792
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1153A
// 首条位置: {"module":1443,"inlinee":70970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12316,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12720,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12148,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13420,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13672,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13516,"parent_record_offset":12032,"ancestor_record_offset":12032,"inlinee":70971,"source_line":972,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":12032,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":12032,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":12032,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":12616,"parent_record_offset":12440,"ancestor_record_offset":12032,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1443,"record_offset":13836,"parent_record_offset":13744,"ancestor_record_offset":12032,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13292,"parent_record_offset":13252,"ancestor_record_offset":12032,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":12032,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12380,"parent_record_offset":12348,"ancestor_record_offset":12032,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":12032,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":12032,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":12032,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":12032,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":12032,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12440,"parent_record_offset":12408,"ancestor_record_offset":12032,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12408,"parent_record_offset":12380,"ancestor_record_offset":12032,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":12032,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1443,"record_offset":12476,"parent_record_offset":12440,"ancestor_record_offset":12032,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13252,"parent_record_offset":12768,"ancestor_record_offset":12032,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":12032,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13168,"parent_record_offset":12768,"ancestor_record_offset":12032,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":13380,"parent_record_offset":12768,"ancestor_record_offset":12032,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":12348,"parent_record_offset":12316,"ancestor_record_offset":12032,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1443,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":12032,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":12032,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1443,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":12032,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1443,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":12032,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":12032,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1443,"record_offset":12172,"parent_record_offset":12148,"ancestor_record_offset":12032,"inlinee":41527,"source_line":118,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1443,"record_offset":12812,"parent_record_offset":12768,"ancestor_record_offset":12032,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1443,"record_offset":13448,"parent_record_offset":13420,"ancestor_record_offset":12032,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12548,"parent_record_offset":12524,"ancestor_record_offset":12032,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":12032,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1443,"record_offset":13804,"parent_record_offset":13772,"ancestor_record_offset":12032,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1443,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":12032,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1443,"record_offset":13076,"parent_record_offset":12812,"ancestor_record_offset":12032,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1443,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":12032,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":12032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":13548,"parent_record_offset":13516,"ancestor_record_offset":12032,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":12244,"parent_record_offset":12220,"ancestor_record_offset":12032,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":13024,"parent_record_offset":13000,"ancestor_record_offset":12032,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":12032,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1443,"record_offset":12948,"parent_record_offset":12912,"ancestor_record_offset":12032,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13576,"parent_record_offset":13516,"ancestor_record_offset":12032,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":13000,"parent_record_offset":12976,"ancestor_record_offset":12032,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":12032,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12912,"parent_record_offset":12884,"ancestor_record_offset":12032,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":12032,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1443,"record_offset":12976,"parent_record_offset":12912,"ancestor_record_offset":12032,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1443,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":12032,"inlinee":70771,"source_line":799,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 799
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11473
// 首条位置: {"module":1415,"inlinee":70771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::WallSide::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 850
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDE
// 首条位置: {"module":395,"inlinee":24542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":33648,"parent_record_offset":33584,"ancestor_record_offset":33584,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":395,"record_offset":33620,"parent_record_offset":33584,"ancestor_record_offset":33584,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides
// 记录形式: compiled_procedure; instances=1
// 源码行: 879
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":395,"record_offset":33356,"rva":7724128,"va":5376433248}
// 编译实例 RVAs: 0x75dc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1355095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e8a952ebff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376436594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35414484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e8d4611c02","size":5,"soft_fail":false,"target_va":5411848096,"va":5376433607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t35106501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e8c5ae1702","size":5,"soft_fail":false,"target_va":5411542064,"va":5376435558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t35106370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e842ae1702","size":5,"soft_fail":false,"target_va":5411541904,"va":5376435529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1594639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e80f551800","size":5,"soft_fail":false,"target_va":5378028128,"va":5376433484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1595028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e894561800","size":5,"soft_fail":false,"target_va":5378028464,"va":5376433431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3983599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e8efc83c00","size":5,"soft_fail":false,"target_va":5380417056,"va":5376433452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smallvec::SmallVec$LT$A$GT$::reserve_one_unchecked
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35208375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e8b73c1902","size":5,"soft_fail":false,"target_va":5411644064,"va":5376435684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5486705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376433248,"raw_hex":"e871b85300","size":5,"soft_fail":false,"target_va":5381920464,"va":5376433754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":395,"record_offset":33524,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":36388,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":33680,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":10136,"source_line":779,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":34196,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":11441,"source_line":1114,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::WallSide::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":33584,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":24542,"source_line":850,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":395,"record_offset":33708,"parent_record_offset":33356,"ancestor_record_offset":33356,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":36700,"parent_record_offset":36676,"ancestor_record_offset":33356,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":395,"record_offset":36676,"parent_record_offset":36592,"ancestor_record_offset":33356,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":36620,"parent_record_offset":36592,"ancestor_record_offset":33356,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":36592,"parent_record_offset":36568,"ancestor_record_offset":33356,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":395,"record_offset":36568,"parent_record_offset":36544,"ancestor_record_offset":33356,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":33648,"parent_record_offset":33584,"ancestor_record_offset":33356,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":33356,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":395,"record_offset":33820,"parent_record_offset":33796,"ancestor_record_offset":33356,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":395,"record_offset":33860,"parent_record_offset":33708,"ancestor_record_offset":33356,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":395,"record_offset":33888,"parent_record_offset":33708,"ancestor_record_offset":33356,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":395,"record_offset":36496,"parent_record_offset":36412,"ancestor_record_offset":33356,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":395,"record_offset":36644,"parent_record_offset":36620,"ancestor_record_offset":33356,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":395,"record_offset":33772,"parent_record_offset":33748,"ancestor_record_offset":33356,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":395,"record_offset":33748,"parent_record_offset":33708,"ancestor_record_offset":33356,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":395,"record_offset":36520,"parent_record_offset":36496,"ancestor_record_offset":33356,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":395,"record_offset":34300,"parent_record_offset":34196,"ancestor_record_offset":33356,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":34268,"parent_record_offset":34244,"ancestor_record_offset":33356,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":34244,"parent_record_offset":34196,"ancestor_record_offset":33356,"inlinee":11432,"source_line":984,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":34336,"parent_record_offset":34196,"ancestor_record_offset":33356,"inlinee":6214,"source_line":659,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":395,"record_offset":36412,"parent_record_offset":36388,"ancestor_record_offset":33356,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":395,"record_offset":33620,"parent_record_offset":33584,"ancestor_record_offset":33356,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":395,"record_offset":33552,"parent_record_offset":33524,"ancestor_record_offset":33356,"inlinee":24540,"source_line":884,"archive":"_pdb/B/modules/00395.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 884
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FDC
// 首条位置: {"module":395,"inlinee":24540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::sd_box
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 964
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FE3
// 首条位置: {"module":1443,"inlinee":24547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":21988,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":22024,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":22056,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":24543,"source_line":320,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21872,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21772,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":22140,"parent_record_offset":21736,"ancestor_record_offset":21736,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":22104,"parent_record_offset":22080,"ancestor_record_offset":21736,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1443,"record_offset":21808,"parent_record_offset":21772,"ancestor_record_offset":21736,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1443,"record_offset":22164,"parent_record_offset":22140,"ancestor_record_offset":21736,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":1443,"record_offset":22080,"parent_record_offset":22056,"ancestor_record_offset":21736,"inlinee":13667,"source_line":146,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1443,"record_offset":21900,"parent_record_offset":21872,"ancestor_record_offset":21736,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1443,"record_offset":21948,"parent_record_offset":21900,"ancestor_record_offset":21736,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 972
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1153B
// 首条位置: {"module":1443,"inlinee":70971}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1443,"record_offset":26064,"parent_record_offset":25960,"ancestor_record_offset":25960,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1443,"record_offset":13548,"parent_record_offset":13516,"ancestor_record_offset":13516,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01443.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1443,"record_offset":13576,"parent_record_offset":13516,"ancestor_record_offset":13516,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01443.jsonl.gz"}
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
// 完整结构及原始类型记录: plaster_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::PlasterBufferOwner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::system::commands::entity_command::insert::closure_env$0", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error_with::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<system_wall_constructor::plaster_systems::PlasterBufferOwner,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner", "bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::entity_command::insert::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::register_info<system_wall_constructor::plaster_systems::PlasterBufferOwner>", "constructor": "bevy_ecs::bundle::Bundles::register_info", "arguments": ["system_wall_constructor::plaster_systems::PlasterBufferOwner"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::WallSide
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "array$<system_wall_constructor::plaster_systems::WallSide,4>", "constructor": "array$", "arguments": ["system_wall_constructor::plaster_systems::WallSide", "4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_wall_constructor::plaster_systems::WallSide,4> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_wall_constructor::plaster_systems::WallSide,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: plaster_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: plaster_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::WallSide::new
// 信息 ID: 07e930905de85b1a15b19504
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=33584; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=34132; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=34792; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=35464; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=36140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster
// 信息 ID: 0863a66f9d520d5cac31e782
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0aa172b68cab4584f80e4193: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> > @ 0x271992 -> 0x75d460 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> > > @ 0x438209 -> 0x75d460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 信息 ID: 4a33e77a960ca34c77c2aad6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::create_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::create_plaster; module=395, inline_record=31384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh
// 信息 ID: 137dd78e773cd7cb421e9b36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=10480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$0
// 信息 ID: 9086e493b561b349af99383c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=11980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$1
// 信息 ID: d53a26502c7cfecf6559ad1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=13980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$2
// 信息 ID: f7f252fabd388285b60be6ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=14036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$3
// 信息 ID: 2e8133ce1f2005176b33e093
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=14404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh::closure$4
// 信息 ID: b76286452c323a43e63e68d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=14864; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=15120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities::closure$0
// 信息 ID: df242b50a8bf14c55e6ee8a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=8912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 信息 ID: 526ab1d79d0d5f00f74f2580
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::filter::Filter -> ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities -> ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x75cae3 -> 0x12269b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::create_plaster @ 0x75da4d -> 0x12269b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::impl$0::drop
// 信息 ID: 6896f4dc0b93a2cafca541b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<system_wall_constructor::plaster_systems::PlasterBufferOwner> @ 0x11fe329 -> 0x11fe600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::impl$2::register_required_components
// 信息 ID: b7f2b9a9485c77c6acec4dff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::impl$2::register_required_components @ 0x75ee7a -> 0x75e9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::bundle::impl$0::component_ids<system_wall_constructor::plaster_systems::PlasterBufferOwner,bevy_ecs::bundle::impl$8::register_info::closure$0::closure_env$0<system_wall_constructor::plaster_systems::PlasterBufferOwner> > @ 0x1226587 -> 0x75e9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal
// 信息 ID: f5c7bb9dd06ebdad61b74678
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=395, inline_record=4908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure$0
// 信息 ID: cf74655523b39311eb64fa9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0 <- alloc::vec::Vec::retain<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global,system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global,system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0>; module=1415, inline_record=8944; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global,system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0>; module=1415, inline_record=9232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster
// 信息 ID: b34be0710efe903c3490e5d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6bb7a5512c9083bddec812b3: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x2720e4 -> 0x75b810 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::plaster::AnimatedPlasterPeel>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x443e1c -> 0x75b810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 信息 ID: 7d2d6a572935e1f9e6c55d2c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::option::IntoIter<alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >,alloc::vec::into_iter::IntoIter<bevy_ecs::entity::Entity,alloc::alloc::Global>,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 1 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$2
// 信息 ID: 54ba40337a519caab7faa1e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=395, inline_record=12524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$3
// 信息 ID: a2e639662101d386c0fcb119
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=395, inline_record=15452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$4
// 信息 ID: d7544f55aebb9323eb6dfd48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::sort8_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::stable::driftsort_main<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4>,alloc::vec::Vec<country_core::resources::walls::WallId,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::stable::drift::sort<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::pivot::median3_rec<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4 <- core::slice::sort::shared::pivot::choose_pivot<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 74 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4488; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4512; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4612; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4636; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4832; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4860; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4952; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::resources::walls::WallId,alloc::slice::impl$0::sort_by_key::closure_env$0<country_core::resources::walls::WallId,usize,system_wall_constructor::plaster_systems::maintain_plaster::closure_env$4> >; module=1444, inline_record=4980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$6
// 信息 ID: da1052d17a48733087a32fb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=395, inline_record=21472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 信息 ID: 533ee19edfd02f235daf2c54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=12032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 信息 ID: 8c0883f9d84c58811d500ba6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::mirror_holes_as_needed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=13124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::sd_box
// 信息 ID: b4518c23c23ef66feacae890
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=21736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides
// 信息 ID: 6d53db00b34e56b5538616f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1226eee -> 0x75dc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 信息 ID: 416cdbf0a6ab17b7160c2ab1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::split_wall_into_sides", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=395, inline_record=33552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 信息 ID: cca7480ebd3c5dd25264f4fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=13516; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=13608; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=13640; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1443, inline_record=25960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
