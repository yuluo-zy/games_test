// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_gates/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bfd7dc44de55b0f3d3e6e751966112abbc49b25eed1f08e813626035893c74a7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1415, 1417, 1431, 1452, 3018
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1431,"record_offset":272,"rva":18940176,"va":5387649296}
// 编译实例 RVAs: 0x1210110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002450
//     owner=unresolved_namespace; count=1
//     target_aliases=418 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18941238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8cafadefe","size":5,"soft_fail":false,"target_va":5368718416,"va":5387659649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18698113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e87fb0e2fe","size":5,"soft_fail":false,"target_va":5368953728,"va":5387651836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18392040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8185ce7fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387659699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17601331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8cd6cf3fe","size":5,"soft_fail":false,"target_va":5370067488,"va":5387668814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16082137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8279b0aff","size":5,"soft_fail":false,"target_va":5371570656,"va":5387652788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12853555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8cdde3bff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387666974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140606810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12637139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e82d2c3fff","size":5,"soft_fail":false,"target_va":5375027216,"va":5387664350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12568182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e88a3940ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387649665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=41
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12568983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8693640ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387650482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10193949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8e37364ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387654056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10194393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8277264ff","size":5,"soft_fail":false,"target_va":5377460160,"va":5387654548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088fe60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9979315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e84dba67ff","size":5,"soft_fail":false,"target_va":5377687136,"va":5387666446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5716691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e82dc5a8ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387651214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-101977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8a771feff","size":5,"soft_fail":false,"target_va":5387551024,"va":5387652996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-103005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8a36dfeff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387656808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-95668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e84c8afeff","size":5,"soft_fail":false,"target_va":5387557152,"va":5387652815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e85e96feff","size":5,"soft_fail":false,"target_va":5387557856,"va":5387650429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fdec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8cfa0feff","size":5,"soft_fail":false,"target_va":5387574976,"va":5387664876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fe280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8b498feff","size":5,"soft_fail":false,"target_va":5387575936,"va":5387667911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141204df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-56396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8b423ffff","size":5,"soft_fail":false,"target_va":5387603440,"va":5387659831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e819abffff","size":5,"soft_fail":false,"target_va":5387632928,"va":5387654658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141237ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t162998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8b67c0200","size":5,"soft_fail":false,"target_va":5387812512,"va":5387649509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123a430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t164010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8aa800200","size":5,"soft_fail":false,"target_va":5387822128,"va":5387658113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t222777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e839660300","size":5,"soft_fail":false,"target_va":5387874224,"va":5387651442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdb840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14456121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e83995dc00","size":5,"soft_fail":false,"target_va":5402114112,"va":5387657986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdb9d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14455370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e84a92dc00","size":5,"soft_fail":false,"target_va":5402114512,"va":5387659137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fdbe50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14456376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e83896dc00","size":5,"soft_fail":false,"target_va":5402115664,"va":5387659283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24196777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8a9367101","size":5,"soft_fail":false,"target_va":5411847168,"va":5387650386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24193388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e86c297101","size":5,"soft_fail":false,"target_va":5411847200,"va":5387653807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24196806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8c6367101","size":5,"soft_fail":false,"target_va":5411848096,"va":5387651285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23876924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e83c556c01","size":5,"soft_fail":false,"target_va":5411540992,"va":5387664063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-6755061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e80bed98ff","size":5,"soft_fail":false,"target_va":5380895104,"va":5387650160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23684885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e815676901","size":5,"soft_fail":false,"target_va":5411339744,"va":5387654854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23877295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8af566c01","size":5,"soft_fail":false,"target_va":5411540963,"va":5387663663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-13534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e822cbffff","size":5,"soft_fail":false,"target_va":5387646912,"va":5387660441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-5227724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8343bb0ff","size":5,"soft_fail":false,"target_va":5382424224,"va":5387651943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t23878227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8535a6c01","size":5,"soft_fail":false,"target_va":5411541776,"va":5387663544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23879835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e89b606c01","size":5,"soft_fail":false,"target_va":5411542064,"va":5387662224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=26
//     evidence={"assembly":"\tcallq\t23879173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8055e6c01","size":5,"soft_fail":false,"target_va":5411542296,"va":5387663118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23879711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e81f606c01","size":5,"soft_fail":false,"target_va":5411541904,"va":5387662188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23878901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8f55c6c01","size":5,"soft_fail":false,"target_va":5411542880,"va":5387663974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23880028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e85c616c01","size":5,"soft_fail":false,"target_va":5411543104,"va":5387663071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9329893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e81ba371ff","size":5,"soft_fail":false,"target_va":5378329696,"va":5387659584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9285320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8385172ff","size":5,"soft_fail":false,"target_va":5378374304,"va":5387659619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMesh::new_with_registry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7776152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8685889ff","size":5,"soft_fail":false,"target_va":5379883520,"va":5387659667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::mesh_from_pts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8021522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8ee9985ff","size":5,"soft_fail":false,"target_va":5379637536,"va":5387659053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e85c2d0000","size":5,"soft_fail":false,"target_va":5387669200,"va":5387657583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11173260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8748255ff","size":5,"soft_fail":false,"target_va":5376481120,"va":5387654375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation_bad
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5765932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387649296,"raw_hex":"e8d404a8ff","size":5,"soft_fail":false,"target_va":5381885904,"va":5387651831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":18712,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":604,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":6952,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":5284,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1431,"record_offset":30352,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":30660,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":18892,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":5836,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":6128,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":5808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":18864,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1431,"record_offset":19368,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":26644,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":26588,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":26448,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":5408,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":4044,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":19100,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":5380,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":28608,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1431,"record_offset":26288,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":19792,"source_line":31,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5532,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5864,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5952,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5444,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":28360,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":29060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":30264,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":29432,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":28244,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":5780,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":24976,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70906,"source_line":308,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_arch_heights
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":4280,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70895,"source_line":514,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_bottom_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":632,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70893,"source_line":455,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":19484,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70901,"source_line":342,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::door_mesh_gap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":22692,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70904,"source_line":404,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":19456,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70900,"source_line":299,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::offset_door_bottom_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":17900,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70898,"source_line":500,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_brick_steps
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":7136,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70897,"source_line":537,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_ladder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":6156,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70896,"source_line":620,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":26796,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":5256,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":18684,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":28808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1448,"parent_record_offset":1416,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":3364,"parent_record_offset":3272,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":272,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":272,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":272,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3300,"parent_record_offset":3272,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6852,"parent_record_offset":6820,"ancestor_record_offset":272,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":272,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":272,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":272,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2852,"parent_record_offset":2816,"ancestor_record_offset":272,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2692,"parent_record_offset":2660,"ancestor_record_offset":272,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2660,"parent_record_offset":2628,"ancestor_record_offset":272,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2628,"parent_record_offset":2596,"ancestor_record_offset":272,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2096,"parent_record_offset":2056,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":272,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":272,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12904,"parent_record_offset":12872,"ancestor_record_offset":272,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12008,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":272,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":11356,"parent_record_offset":11320,"ancestor_record_offset":272,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":7436,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2056,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":6764,"parent_record_offset":6728,"ancestor_record_offset":272,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10008,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9716,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":1292,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":1324,"parent_record_offset":1292,"ancestor_record_offset":272,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2596,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":3672,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12848,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9340,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":272,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":272,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1431,"record_offset":31024,"parent_record_offset":31000,"ancestor_record_offset":272,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1431,"record_offset":31072,"parent_record_offset":31048,"ancestor_record_offset":272,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1431,"record_offset":10628,"parent_record_offset":10040,"ancestor_record_offset":272,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":272,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6728,"parent_record_offset":6696,"ancestor_record_offset":272,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6608,"parent_record_offset":6156,"ancestor_record_offset":272,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":272,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6668,"parent_record_offset":6640,"ancestor_record_offset":272,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1431,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":272,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1431,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":272,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":272,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":7692,"parent_record_offset":7624,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":272,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1431,"record_offset":6552,"parent_record_offset":6156,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":21152,"parent_record_offset":20980,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":1888,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":1804,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1431,"record_offset":8252,"parent_record_offset":8108,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1431,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":272,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":272,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":272,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":272,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1431,"record_offset":2312,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":272,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":272,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":20112,"parent_record_offset":20068,"ancestor_record_offset":272,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":996,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":272,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1728,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":272,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":272,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7740,"parent_record_offset":7624,"ancestor_record_offset":272,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7624,"parent_record_offset":7576,"ancestor_record_offset":272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7576,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1431,"record_offset":23660,"parent_record_offset":23212,"ancestor_record_offset":272,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":272,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":1240,"parent_record_offset":996,"ancestor_record_offset":272,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":23688,"parent_record_offset":23212,"ancestor_record_offset":272,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":7980,"parent_record_offset":7864,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":884,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":1588,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":272,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1431,"record_offset":23568,"parent_record_offset":23544,"ancestor_record_offset":272,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":272,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":272,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":7780,"parent_record_offset":7740,"ancestor_record_offset":272,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":9624,"parent_record_offset":9592,"ancestor_record_offset":272,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1431,"record_offset":1832,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":41401,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1431,"record_offset":20268,"parent_record_offset":20240,"ancestor_record_offset":272,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":20240,"parent_record_offset":20112,"ancestor_record_offset":272,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":20532,"parent_record_offset":20508,"ancestor_record_offset":272,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":1552,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":23744,"parent_record_offset":23212,"ancestor_record_offset":272,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1431,"record_offset":3092,"parent_record_offset":2692,"ancestor_record_offset":272,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":3200,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":272,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":30412,"parent_record_offset":30352,"ancestor_record_offset":272,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":272,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":18820,"parent_record_offset":18792,"ancestor_record_offset":272,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":2216,"parent_record_offset":2056,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":3164,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":8064,"parent_record_offset":8036,"ancestor_record_offset":272,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":9744,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":2248,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":18060,"parent_record_offset":17900,"ancestor_record_offset":272,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":272,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":272,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":272,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1431,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":23444,"parent_record_offset":23420,"ancestor_record_offset":272,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":272,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":23420,"parent_record_offset":23328,"ancestor_record_offset":272,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9424,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9504,"parent_record_offset":9424,"ancestor_record_offset":272,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1431,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":272,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1431,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":272,"inlinee":13110,"source_line":43,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1431,"record_offset":12292,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":14290,"source_line":201,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5620,"parent_record_offset":5588,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5588,"parent_record_offset":5556,"ancestor_record_offset":272,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":272,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19884,"parent_record_offset":19484,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18340,"parent_record_offset":17900,"ancestor_record_offset":272,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18180,"parent_record_offset":17900,"ancestor_record_offset":272,"inlinee":25426,"source_line":481,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":9592,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18124,"parent_record_offset":17900,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":272,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1164,"parent_record_offset":1076,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":27516,"parent_record_offset":27484,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":27484,"parent_record_offset":27456,"ancestor_record_offset":272,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1944,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":27456,"parent_record_offset":27428,"ancestor_record_offset":272,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":6312,"parent_record_offset":6156,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":18540,"parent_record_offset":17900,"ancestor_record_offset":272,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":20484,"parent_record_offset":20068,"ancestor_record_offset":272,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":23544,"parent_record_offset":23212,"ancestor_record_offset":272,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":20068,"parent_record_offset":19484,"ancestor_record_offset":272,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1431,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":1196,"parent_record_offset":1164,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":20296,"parent_record_offset":20268,"ancestor_record_offset":272,"inlinee":70903,"source_line":354,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":776,"parent_record_offset":632,"ancestor_record_offset":272,"inlinee":70894,"source_line":528,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":272,"inlinee":70834,"source_line":533,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1431,"record_offset":7548,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":14282,"source_line":50,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":8036,"parent_record_offset":7864,"ancestor_record_offset":272,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":8460,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":24574,"source_line":851,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":7864,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":26828,"parent_record_offset":26796,"ancestor_record_offset":272,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":4348,"parent_record_offset":4280,"ancestor_record_offset":272,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21464,"parent_record_offset":21424,"ancestor_record_offset":272,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21556,"parent_record_offset":21424,"ancestor_record_offset":272,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1431,"record_offset":8108,"parent_record_offset":7136,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21424,"parent_record_offset":19484,"ancestor_record_offset":272,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1431,"record_offset":27428,"parent_record_offset":27024,"ancestor_record_offset":272,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1148A
// 首条位置: {"module":1415,"inlinee":70794}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1415,"record_offset":67812,"parent_record_offset":67636,"ancestor_record_offset":67636,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1415,"record_offset":67668,"parent_record_offset":67636,"ancestor_record_offset":67636,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1415,"record_offset":67724,"parent_record_offset":67636,"ancestor_record_offset":67636,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1415,"record_offset":67836,"parent_record_offset":67812,"ancestor_record_offset":67636,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1415,"record_offset":67752,"parent_record_offset":67724,"ancestor_record_offset":67636,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1415,"record_offset":67692,"parent_record_offset":67668,"ancestor_record_offset":67636,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 299
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F4
// 首条位置: {"module":1431,"inlinee":70900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114FA
// 首条位置: {"module":1431,"inlinee":70906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":25116,"parent_record_offset":24976,"ancestor_record_offset":24976,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":25032,"parent_record_offset":24976,"ancestor_record_offset":24976,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":25300,"parent_record_offset":25276,"ancestor_record_offset":24976,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":25276,"parent_record_offset":25188,"ancestor_record_offset":24976,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":25216,"parent_record_offset":25188,"ancestor_record_offset":24976,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":25188,"parent_record_offset":25164,"ancestor_record_offset":24976,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":25164,"parent_record_offset":25140,"ancestor_record_offset":24976,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":25244,"parent_record_offset":25216,"ancestor_record_offset":24976,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":25140,"parent_record_offset":25116,"ancestor_record_offset":24976,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":25080,"parent_record_offset":25056,"ancestor_record_offset":24976,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1431,"record_offset":25056,"parent_record_offset":25032,"ancestor_record_offset":24976,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
// 记录形式: compiled_procedure; instances=1
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1431,"record_offset":33372,"rva":18960080,"va":5387669200}
// 编译实例 RVAs: 0x1214ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=24
//     evidence={"assembly":"\tcallq\t-6774158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387669200,"raw_hex":"e872a298ff","size":5,"soft_fail":false,"target_va":5380895104,"va":5387669257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":33532,"parent_record_offset":33372,"ancestor_record_offset":33372,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":33820,"parent_record_offset":33372,"ancestor_record_offset":33372,"inlinee":44691,"source_line":121,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":33668,"parent_record_offset":33604,"ancestor_record_offset":33372,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":33732,"parent_record_offset":33700,"ancestor_record_offset":33372,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":33700,"parent_record_offset":33668,"ancestor_record_offset":33372,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":33604,"parent_record_offset":33532,"ancestor_record_offset":33372,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":33780,"parent_record_offset":33532,"ancestor_record_offset":33372,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F5
// 首条位置: {"module":1431,"inlinee":70901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":19668,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":19792,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":19764,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19884,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":20040,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19828,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":22540,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":22596,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":20068,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1431,"record_offset":20980,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21424,"parent_record_offset":19484,"ancestor_record_offset":19484,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":21080,"parent_record_offset":21052,"ancestor_record_offset":19484,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":21108,"parent_record_offset":21080,"ancestor_record_offset":19484,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":20200,"parent_record_offset":20172,"ancestor_record_offset":19484,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":21152,"parent_record_offset":20980,"ancestor_record_offset":19484,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1431,"record_offset":21180,"parent_record_offset":20980,"ancestor_record_offset":19484,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":20508,"parent_record_offset":20484,"ancestor_record_offset":19484,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":20556,"parent_record_offset":20532,"ancestor_record_offset":19484,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":20112,"parent_record_offset":20068,"ancestor_record_offset":19484,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":20172,"parent_record_offset":20144,"ancestor_record_offset":19484,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":20144,"parent_record_offset":20112,"ancestor_record_offset":19484,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":20580,"parent_record_offset":20556,"ancestor_record_offset":19484,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":20436,"parent_record_offset":20328,"ancestor_record_offset":19484,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":20380,"parent_record_offset":20328,"ancestor_record_offset":19484,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":19852,"parent_record_offset":19828,"ancestor_record_offset":19484,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1431,"record_offset":20268,"parent_record_offset":20240,"ancestor_record_offset":19484,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":19484,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":21024,"parent_record_offset":20980,"ancestor_record_offset":19484,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":20240,"parent_record_offset":20112,"ancestor_record_offset":19484,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":20532,"parent_record_offset":20508,"ancestor_record_offset":19484,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":19696,"parent_record_offset":19668,"ancestor_record_offset":19484,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":19724,"parent_record_offset":19696,"ancestor_record_offset":19484,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19964,"parent_record_offset":19932,"ancestor_record_offset":19484,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19932,"parent_record_offset":19908,"ancestor_record_offset":19484,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":19484,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":22356,"parent_record_offset":22048,"ancestor_record_offset":19484,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":22444,"parent_record_offset":22048,"ancestor_record_offset":19484,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":22264,"parent_record_offset":22048,"ancestor_record_offset":19484,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":20484,"parent_record_offset":20068,"ancestor_record_offset":19484,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1431,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":19484,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":19996,"parent_record_offset":19964,"ancestor_record_offset":19484,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::door_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":20296,"parent_record_offset":20268,"ancestor_record_offset":19484,"inlinee":70903,"source_line":354,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21464,"parent_record_offset":21424,"ancestor_record_offset":19484,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":21556,"parent_record_offset":21424,"ancestor_record_offset":19484,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 354
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F7
// 首条位置: {"module":1431,"inlinee":70903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1431,"record_offset":20328,"parent_record_offset":20296,"ancestor_record_offset":20296,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":20436,"parent_record_offset":20328,"ancestor_record_offset":20296,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":20380,"parent_record_offset":20328,"ancestor_record_offset":20296,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":20352,"parent_record_offset":20328,"ancestor_record_offset":20296,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F8
// 首条位置: {"module":1431,"inlinee":70904}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":22812,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":22936,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":22908,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":23028,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":23184,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":22972,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":24740,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":24796,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":23212,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1431,"record_offset":23776,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":24200,"parent_record_offset":22692,"ancestor_record_offset":22692,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":23864,"parent_record_offset":23840,"ancestor_record_offset":22692,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":22692,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":23928,"parent_record_offset":23776,"ancestor_record_offset":22692,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1431,"record_offset":23956,"parent_record_offset":23776,"ancestor_record_offset":22692,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1431,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":22692,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1431,"record_offset":23292,"parent_record_offset":23264,"ancestor_record_offset":22692,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":23264,"parent_record_offset":23212,"ancestor_record_offset":22692,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":1431,"record_offset":23660,"parent_record_offset":23212,"ancestor_record_offset":22692,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":23688,"parent_record_offset":23212,"ancestor_record_offset":22692,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1431,"record_offset":23568,"parent_record_offset":23544,"ancestor_record_offset":22692,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":24708,"parent_record_offset":24424,"ancestor_record_offset":22692,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":24680,"parent_record_offset":24424,"ancestor_record_offset":22692,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":22996,"parent_record_offset":22972,"ancestor_record_offset":22692,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":23840,"parent_record_offset":23816,"ancestor_record_offset":22692,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":23816,"parent_record_offset":23776,"ancestor_record_offset":22692,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":23512,"parent_record_offset":23264,"ancestor_record_offset":22692,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":23744,"parent_record_offset":23212,"ancestor_record_offset":22692,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":23468,"parent_record_offset":23444,"ancestor_record_offset":22692,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":23384,"parent_record_offset":23328,"ancestor_record_offset":22692,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":22692,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":23624,"parent_record_offset":23568,"ancestor_record_offset":22692,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":22840,"parent_record_offset":22812,"ancestor_record_offset":22692,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":22868,"parent_record_offset":22840,"ancestor_record_offset":22692,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":23444,"parent_record_offset":23420,"ancestor_record_offset":22692,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":23328,"parent_record_offset":23292,"ancestor_record_offset":22692,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":23420,"parent_record_offset":23328,"ancestor_record_offset":22692,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":23108,"parent_record_offset":23076,"ancestor_record_offset":22692,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22692,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":23052,"parent_record_offset":23028,"ancestor_record_offset":22692,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1431,"record_offset":23544,"parent_record_offset":23212,"ancestor_record_offset":22692,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":23140,"parent_record_offset":23108,"ancestor_record_offset":22692,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":24236,"parent_record_offset":24200,"ancestor_record_offset":22692,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":24328,"parent_record_offset":24200,"ancestor_record_offset":22692,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E7B
// 首条位置: {"module":3018,"inlinee":106107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3018,"record_offset":1280,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3018,"record_offset":1368,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3018,"record_offset":556,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3018,"record_offset":764,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3018,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":508,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3018,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":508,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3018,"record_offset":704,"parent_record_offset":556,"ancestor_record_offset":508,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3018,"record_offset":732,"parent_record_offset":556,"ancestor_record_offset":508,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3018,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":508,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3018,"record_offset":1128,"parent_record_offset":1100,"ancestor_record_offset":508,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3018,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":508,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3018,"record_offset":1040,"parent_record_offset":1012,"ancestor_record_offset":508,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3018,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":508,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3018,"record_offset":596,"parent_record_offset":556,"ancestor_record_offset":508,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3018,"record_offset":840,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3018,"record_offset":872,"parent_record_offset":840,"ancestor_record_offset":508,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3018,"record_offset":1100,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3018,"record_offset":1188,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3018,"record_offset":1012,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3018,"record_offset":812,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3018,"record_offset":912,"parent_record_offset":764,"ancestor_record_offset":508,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/03018.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_bottom_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114ED
// 首条位置: {"module":1431,"inlinee":70893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2056,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":1292,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2596,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":3672,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":1888,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":1804,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1431,"record_offset":2312,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1728,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":1588,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$56::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1431,"record_offset":1832,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":41401,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":1552,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":3200,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":3164,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":2248,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":2400,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1944,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::segment_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":776,"parent_record_offset":632,"ancestor_record_offset":632,"inlinee":70894,"source_line":528,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":632,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":632,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":632,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":1448,"parent_record_offset":1416,"ancestor_record_offset":632,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":3364,"parent_record_offset":3272,"ancestor_record_offset":632,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2128,"parent_record_offset":2096,"ancestor_record_offset":632,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1356,"parent_record_offset":1324,"ancestor_record_offset":632,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3300,"parent_record_offset":3272,"ancestor_record_offset":632,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":632,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":632,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":632,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":632,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":632,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":632,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":2852,"parent_record_offset":2816,"ancestor_record_offset":632,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2692,"parent_record_offset":2660,"ancestor_record_offset":632,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2660,"parent_record_offset":2628,"ancestor_record_offset":632,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1431,"record_offset":2628,"parent_record_offset":2596,"ancestor_record_offset":632,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":2096,"parent_record_offset":2056,"ancestor_record_offset":632,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":1324,"parent_record_offset":1292,"ancestor_record_offset":632,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1431,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":632,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1431,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":632,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":632,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":632,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1431,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":632,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":632,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":996,"parent_record_offset":956,"ancestor_record_offset":632,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":632,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":632,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":632,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":632,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":1240,"parent_record_offset":996,"ancestor_record_offset":632,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":632,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":884,"parent_record_offset":776,"ancestor_record_offset":632,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":632,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":632,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":3332,"parent_record_offset":3300,"ancestor_record_offset":632,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":632,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":632,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":632,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1431,"record_offset":3092,"parent_record_offset":2692,"ancestor_record_offset":632,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":632,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":632,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":2216,"parent_record_offset":2056,"ancestor_record_offset":632,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":632,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":632,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":632,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":632,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1431,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":632,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":632,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":632,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1164,"parent_record_offset":1076,"ancestor_record_offset":632,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":632,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":632,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":1196,"parent_record_offset":1164,"ancestor_record_offset":632,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":632,"inlinee":70834,"source_line":533,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::offset_door_bottom_curve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 500
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F2
// 首条位置: {"module":1431,"inlinee":70898}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":17964,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1431,"record_offset":18088,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":18060,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":18368,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18340,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18180,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":25426,"source_line":481,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18124,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":18540,"parent_record_offset":17900,"ancestor_record_offset":17900,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":18588,"parent_record_offset":18540,"ancestor_record_offset":17900,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":18148,"parent_record_offset":18124,"ancestor_record_offset":17900,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":18500,"parent_record_offset":18476,"ancestor_record_offset":17900,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":18420,"parent_record_offset":18368,"ancestor_record_offset":17900,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":17992,"parent_record_offset":17964,"ancestor_record_offset":17900,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":18020,"parent_record_offset":17992,"ancestor_record_offset":17900,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":17900,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":18452,"parent_record_offset":18368,"ancestor_record_offset":17900,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18264,"parent_record_offset":18232,"ancestor_record_offset":17900,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":17900,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":18208,"parent_record_offset":18180,"ancestor_record_offset":17900,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":18296,"parent_record_offset":18264,"ancestor_record_offset":17900,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::door_arch_heights
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 514
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114EF
// 首条位置: {"module":1431,"inlinee":70895}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":4636,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":4376,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":4900,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1431,"record_offset":4816,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":4996,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":4348,"parent_record_offset":4280,"ancestor_record_offset":4280,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":4280,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":4280,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4280,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":4280,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":5172,"parent_record_offset":5072,"ancestor_record_offset":4280,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4280,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":4280,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":5104,"parent_record_offset":5072,"ancestor_record_offset":4280,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":4280,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4744,"parent_record_offset":4720,"ancestor_record_offset":4280,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":4280,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":4280,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":4468,"parent_record_offset":4440,"ancestor_record_offset":4280,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":4280,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":4672,"parent_record_offset":4636,"ancestor_record_offset":4280,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":4408,"parent_record_offset":4376,"ancestor_record_offset":4280,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":4280,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1431,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":4280,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":5136,"parent_record_offset":5104,"ancestor_record_offset":4280,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":4280,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":4784,"parent_record_offset":4636,"ancestor_record_offset":4280,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":4928,"parent_record_offset":4900,"ancestor_record_offset":4280,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4280,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114EE
// 首条位置: {"module":1431,"inlinee":70894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":884,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":776,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":776,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1431,"record_offset":996,"parent_record_offset":956,"ancestor_record_offset":776,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":776,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":1431,"record_offset":1240,"parent_record_offset":996,"ancestor_record_offset":776,"inlinee":17981,"source_line":109,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":776,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":776,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":776,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1431,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":776,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":776,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1164,"parent_record_offset":1076,"ancestor_record_offset":776,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":776,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":1196,"parent_record_offset":1164,"ancestor_record_offset":776,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_gates::segment_length::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1431,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":776,"inlinee":70834,"source_line":533,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::segment_length::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 533
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114B2
// 首条位置: {"module":1417,"inlinee":70834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":1024,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":1024,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1164,"parent_record_offset":1076,"ancestor_record_offset":1024,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":1024,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1431,"record_offset":1196,"parent_record_offset":1164,"ancestor_record_offset":1024,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_brick_steps
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 537
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F1
// 首条位置: {"module":1431,"inlinee":70897}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12008,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":7436,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10008,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9716,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4915,"source_line":1524,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12092,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12848,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9340,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1431,"record_offset":12036,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1431,"record_offset":7836,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7576,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":8840,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":9744,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":8645,"source_line":3567,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":13108,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9424,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1431,"record_offset":12292,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":14290,"source_line":201,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1431,"record_offset":9592,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":14288,"source_line":1317,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1431,"record_offset":7548,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":14282,"source_line":50,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":8460,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":24574,"source_line":851,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":7864,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":9688,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1431,"record_offset":8108,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":8788,"parent_record_offset":8764,"ancestor_record_offset":7136,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1431,"record_offset":8764,"parent_record_offset":8672,"ancestor_record_offset":7136,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10324,"parent_record_offset":10272,"ancestor_record_offset":7136,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":7136,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":7136,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":8700,"parent_record_offset":8672,"ancestor_record_offset":7136,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":8672,"parent_record_offset":8648,"ancestor_record_offset":7136,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10196,"parent_record_offset":10164,"ancestor_record_offset":7136,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10380,"parent_record_offset":10352,"ancestor_record_offset":7136,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10352,"parent_record_offset":10324,"ancestor_record_offset":7136,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":10164,"parent_record_offset":10136,"ancestor_record_offset":7136,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":8648,"parent_record_offset":8624,"ancestor_record_offset":7136,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10272,"parent_record_offset":10040,"ancestor_record_offset":7136,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":7136,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":7136,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12904,"parent_record_offset":12872,"ancestor_record_offset":7136,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":7136,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":11356,"parent_record_offset":11320,"ancestor_record_offset":7136,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":8504,"parent_record_offset":8460,"ancestor_record_offset":7136,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":10108,"parent_record_offset":10040,"ancestor_record_offset":7136,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":7136,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":7136,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":7136,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1431,"record_offset":10628,"parent_record_offset":10040,"ancestor_record_offset":7136,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":7136,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":7136,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1431,"record_offset":7692,"parent_record_offset":7624,"ancestor_record_offset":7136,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1431,"record_offset":8252,"parent_record_offset":8108,"ancestor_record_offset":7136,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7740,"parent_record_offset":7624,"ancestor_record_offset":7136,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":7624,"parent_record_offset":7576,"ancestor_record_offset":7136,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1431,"record_offset":10424,"parent_record_offset":10040,"ancestor_record_offset":7136,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1431,"record_offset":7980,"parent_record_offset":7864,"ancestor_record_offset":7136,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":7780,"parent_record_offset":7740,"ancestor_record_offset":7136,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1431,"record_offset":8732,"parent_record_offset":8700,"ancestor_record_offset":7136,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":9624,"parent_record_offset":9592,"ancestor_record_offset":7136,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":7136,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1431,"record_offset":8140,"parent_record_offset":8108,"ancestor_record_offset":7136,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":8600,"parent_record_offset":8460,"ancestor_record_offset":7136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1431,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":7136,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":9552,"parent_record_offset":9528,"ancestor_record_offset":7136,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1431,"record_offset":9472,"parent_record_offset":9424,"ancestor_record_offset":7136,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1431,"record_offset":10540,"parent_record_offset":10040,"ancestor_record_offset":7136,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1431,"record_offset":8064,"parent_record_offset":8036,"ancestor_record_offset":7136,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1431,"record_offset":7896,"parent_record_offset":7864,"ancestor_record_offset":7136,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":12120,"parent_record_offset":12092,"ancestor_record_offset":7136,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1431,"record_offset":12148,"parent_record_offset":12120,"ancestor_record_offset":7136,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1431,"record_offset":7920,"parent_record_offset":7896,"ancestor_record_offset":7136,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":7136,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9804,"parent_record_offset":9768,"ancestor_record_offset":7136,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1431,"record_offset":9504,"parent_record_offset":9424,"ancestor_record_offset":7136,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\specialize.rs
//     evidence={"module":1431,"record_offset":9768,"parent_record_offset":9744,"ancestor_record_offset":7136,"inlinee":8647,"source_line":18,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1431,"record_offset":8036,"parent_record_offset":7864,"ancestor_record_offset":7136,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_gates::place_ladder
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 620
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x114F0
// 首条位置: {"module":1431,"inlinee":70896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6608,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1431,"record_offset":6552,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":6224,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":6312,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1431,"record_offset":6432,"parent_record_offset":6156,"ancestor_record_offset":6156,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6792,"parent_record_offset":6764,"ancestor_record_offset":6156,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":6156,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6852,"parent_record_offset":6820,"ancestor_record_offset":6156,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1431,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":6156,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1431,"record_offset":6764,"parent_record_offset":6728,"ancestor_record_offset":6156,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6696,"parent_record_offset":6668,"ancestor_record_offset":6156,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6728,"parent_record_offset":6696,"ancestor_record_offset":6156,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6640,"parent_record_offset":6608,"ancestor_record_offset":6156,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":1431,"record_offset":6668,"parent_record_offset":6640,"ancestor_record_offset":6156,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":6340,"parent_record_offset":6312,"ancestor_record_offset":6156,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":6156,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1431,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":6156,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01431.jsonl.gz"}
//   [pdb_inline_transitive] country_core::components::drawable::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/components/drawable.rs
//     evidence={"module":1431,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":6156,"inlinee":13110,"source_line":43,"archive":"_pdb/B/modules/01431.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_gates::DoorMeshInProgress
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_gates::DoorMeshInProgress>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_gates::DoorMeshInProgress"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/B-core/registration-types-wide.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::construct_gates::DoorMeshInProgress", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/B-mesh-helpers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "system_wall_constructor::construct_gates::DoorMeshInProgress", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_gates/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::add_quad
// 信息 ID: 124cfb0475b4bf878668d4aa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x121216f -> 0x1214ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1212643 -> 0x1214ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::DoorMeshInProgress::into_mesh
// 信息 ID: 86f917dfb64ba07b0e8876dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=24976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors
// 信息 ID: f1ea0b61892ca44a7a8d4936
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x756da0 -> 0x1210110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure$0
// 信息 ID: 03d1ae3b0df71c27bbb77f87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::construct_wooden_gate_doors", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::construct_wooden_gate_doors::closure_env$0> >; module=1415, inline_record=67636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_arch_heights
// 信息 ID: b4ea41ef95c95b6e8c9fef70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=4280; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_bottom_curve
// 信息 ID: 38911c9b79c6bfced4220a9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh
// 信息 ID: 94c1ccb47131801b1e70bba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=19484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh::closure$0
// 信息 ID: b82bddfa3c14135301cbcedd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=20296; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=20768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap
// 信息 ID: 7d6f2d689ad65aded6b39b19
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=22692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::door_mesh_gap::closure$0
// 信息 ID: 60469e10a8d8413bcc7fe40b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::door_mesh_gap", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<glam::f32::vec3::Vec3,f32>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<glam::f32::vec3::Vec3,f32> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<f32> >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<f32> > >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<f32> > >,system_wall_constructor::construct_gates::door_mesh_gap::closure_env$0>; module=3018, inline_record=508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::impl$1::default
// 信息 ID: db0945297ba9deb7b0a9f8b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=19456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::offset_door_bottom_curve
// 信息 ID: 534047ac3ad3a60f1603c9e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=17900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_brick_steps
// 信息 ID: 364432410834e71cd157a533
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=7136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::place_ladder
// 信息 ID: 862f259d00fee7d95fba2d02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=6156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length
// 信息 ID: f9d790592418c410b681f749
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_gates::segment_length::closure$0
// 信息 ID: 358a01d90bd194c78e1dcdbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_gates::segment_length", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1431, inline_record=1024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
