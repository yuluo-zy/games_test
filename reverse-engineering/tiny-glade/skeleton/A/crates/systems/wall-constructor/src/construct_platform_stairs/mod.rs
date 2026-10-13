// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4203d17581bb1b70d50a6c76861b7cf63f6e5060bc17d7c75fb80a90e3157c9f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 373, 1226, 1264
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
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::construct_platform_stairs
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":373,"record_offset":260,"rva":5174448,"va":5373883568}
// 编译实例 RVAs: 0x4ef4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5173539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8dd0eb1ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5373894206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4270830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e812d5beff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373884345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2233089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8ffecddff","size":5,"soft_fail":false,"target_va":5371651744,"va":5373884828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1470411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e83590e9ff","size":5,"soft_fail":false,"target_va":5372425312,"va":5373895718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1135509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e86baceeff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373885200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=39
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1136049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e84faaeeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373885756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140589ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t623534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8ae830900","size":5,"soft_fail":false,"target_va":5374516448,"va":5373892909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=6
//     target_aliases=14 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1984461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8cd471e00","size":5,"soft_fail":false,"target_va":5375881440,"va":5373896974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ecd20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2073133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e82da21f00","size":5,"soft_fail":false,"target_va":5375970592,"va":5373897454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3056268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e88ca22e00","size":5,"soft_fail":false,"target_va":5376944000,"va":5373887727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8031484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8fc8c7a00","size":5,"soft_fail":false,"target_va":5381915200,"va":5373883711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8026999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8777b7a00","size":5,"soft_fail":false,"target_va":5381918976,"va":5373891972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca9360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8094199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8f7817b00","size":5,"soft_fail":false,"target_va":5381985120,"va":5373890916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cab1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8108872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e848bb7b00","size":5,"soft_fail":false,"target_va":5381992912,"va":5373884035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8106794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e82ab37b00","size":5,"soft_fail":false,"target_va":5381996400,"va":5373889601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cad2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8108897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e861bb7b00","size":5,"soft_fail":false,"target_va":5382001376,"va":5373892474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb42d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8141291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8eb397c00","size":5,"soft_fail":false,"target_va":5382030032,"va":5373888736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8144845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8cd477c00","size":5,"soft_fail":false,"target_va":5382031392,"va":5373886542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8143349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8f5417c00","size":5,"soft_fail":false,"target_va":5382032368,"va":5373889014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdcd0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8184665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e859e37c00","size":5,"soft_fail":false,"target_va":5382069456,"va":5373884786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbded0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8176975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e84fc57c00","size":5,"soft_fail":false,"target_va":5382069968,"va":5373892988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29709558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8f654c501","size":5,"soft_fail":false,"target_va":5403593344,"va":5373883781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29704752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e83042c501","size":5,"soft_fail":false,"target_va":5403593360,"va":5373888603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29703148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8ec3bc501","size":5,"soft_fail":false,"target_va":5403593392,"va":5373890239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29706916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8a44ac501","size":5,"soft_fail":false,"target_va":5403594096,"va":5373887175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29708139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e86b4fc501","size":5,"soft_fail":false,"target_va":5403594144,"va":5373886000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29708432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e89050c501","size":5,"soft_fail":false,"target_va":5403594288,"va":5373885851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29083350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8d6c6bb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373894789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t29165304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8f806bd01","size":5,"soft_fail":false,"target_va":5403053728,"va":5373888419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3632316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8bc6c3700","size":5,"soft_fail":false,"target_va":5377527120,"va":5373894799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t29084125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8ddc9bb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373894750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29159429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e805f0bc01","size":5,"soft_fail":false,"target_va":5403054624,"va":5373895190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t29084414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8fecabb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373894733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t29084752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e850ccbb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373894587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29083993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e859c9bb01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373894946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29084701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e81dccbb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373895422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29084966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e826cdbb01","size":5,"soft_fail":false,"target_va":5402980304,"va":5373895333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7983234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e882d07900","size":5,"soft_fail":false,"target_va":5381870304,"va":5373887065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1082021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8a5821000","size":5,"soft_fail":false,"target_va":5374975200,"va":5373893174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t8046206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e87ec67a00","size":5,"soft_fail":false,"target_va":5381932864,"va":5373886653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoles::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1271921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e871681300","size":5,"soft_fail":false,"target_va":5375166208,"va":5373894282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1232422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e826ce1200","size":5,"soft_fail":false,"target_va":5375118128,"va":5373885701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1232524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e88cce1200","size":5,"soft_fail":false,"target_va":5375118352,"va":5373885823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::backend::flush_screen_log::log_error_to_screen
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1788550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8864a1b00","size":5,"soft_fail":false,"target_va":5375682896,"va":5373894341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t624040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8a8850900","size":5,"soft_fail":false,"target_va":5374516688,"va":5373892643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::skewed_box_trigger_vol
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t397229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8ad0f0600","size":5,"soft_fail":false,"target_va":5374285616,"va":5373888382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1835106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e862001c00","size":5,"soft_fail":false,"target_va":5375723824,"va":5373888713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t29083483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e85bc7bb01","size":5,"soft_fail":false,"target_va":5402968992,"va":5373885504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t29085006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e84ecdbb01","size":5,"soft_fail":false,"target_va":5402970800,"va":5373885789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8297366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8969b7e00","size":5,"soft_fail":false,"target_va":5382189152,"va":5373891781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_closest_u_from_pos
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4078653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e83d3c3e00","size":5,"soft_fail":false,"target_va":5377968032,"va":5373889374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4077641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e849383e00","size":5,"soft_fail":false,"target_va":5377968384,"va":5373890738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::circular_slice
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4074566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8462c3e00","size":5,"soft_fail":false,"target_va":5377963808,"va":5373889237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4073310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e85e273e00","size":5,"soft_fail":false,"target_va":5377961984,"va":5373888669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4066849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8210e3e00","size":5,"soft_fail":false,"target_va":5377957696,"va":5373890842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3053650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e852982e00","size":5,"soft_fail":false,"target_va":5376939616,"va":5373885961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8369825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8a1b67f00","size":5,"soft_fail":false,"target_va":5382255664,"va":5373885834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::from_world_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8362403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373883568,"raw_hex":"e8a3997f00","size":5,"soft_fail":false,"target_va":5382256368,"va":5373893960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373883568,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5373883801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":13620,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":373,"record_offset":13772,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":18152,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1076,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":932,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":15260,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":15020,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":4728,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":16428,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":18180,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1424,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":5456,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":373,"record_offset":1480,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":373,"record_offset":1016,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":373,"record_offset":8676,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":373,"record_offset":10200,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":373,"record_offset":10228,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":373,"record_offset":10144,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":373,"record_offset":16992,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":373,"record_offset":16204,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":373,"record_offset":1312,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":373,"record_offset":896,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":3688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":15048,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12289,"source_line":3475,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":6576,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":6976,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":26624,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":373,"record_offset":5572,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":373,"record_offset":656,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":373,"record_offset":5648,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$18::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":373,"record_offset":18444,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21588,"source_line":201,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":13316,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":9748,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":10056,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":16936,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14662,"source_line":1012,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":9968,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":373,"record_offset":7060,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":373,"record_offset":7180,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":373,"record_offset":16288,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":373,"record_offset":24240,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":373,"record_offset":1628,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":373,"record_offset":23252,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":373,"record_offset":23584,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":373,"record_offset":10172,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":373,"record_offset":6548,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::add_colision_geo
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":373,"record_offset":11040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21584,"source_line":443,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::update_phys
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":373,"record_offset":20960,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21589,"source_line":414,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":373,"record_offset":1396,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21523,"source_line":51,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":8900,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":9472,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":26692,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":13856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":17320,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_new_from_points_remove_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":17056,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21586,"source_line":221,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":373,"record_offset":8248,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":373,"record_offset":6640,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":6668,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":7208,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21574,"source_line":55,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":6948,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21573,"source_line":9,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":21348,"parent_record_offset":21312,"ancestor_record_offset":260,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":4944,"parent_record_offset":4836,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":6300,"parent_record_offset":6200,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":373,"record_offset":21312,"parent_record_offset":21280,"ancestor_record_offset":260,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":260,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":260,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":4836,"parent_record_offset":4800,"ancestor_record_offset":260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":4800,"parent_record_offset":4764,"ancestor_record_offset":260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":5064,"parent_record_offset":4800,"ancestor_record_offset":260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":373,"record_offset":16512,"parent_record_offset":16484,"ancestor_record_offset":260,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":373,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":260,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":373,"record_offset":16484,"parent_record_offset":16456,"ancestor_record_offset":260,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":373,"record_offset":16456,"parent_record_offset":16428,"ancestor_record_offset":260,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":3820,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":21280,"parent_record_offset":20960,"ancestor_record_offset":260,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":3784,"parent_record_offset":3760,"ancestor_record_offset":260,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":260,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12464,"parent_record_offset":12276,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1112,"parent_record_offset":1076,"ancestor_record_offset":260,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":15288,"parent_record_offset":15260,"ancestor_record_offset":260,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":260,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":21052,"parent_record_offset":20960,"ancestor_record_offset":260,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1168,"parent_record_offset":1076,"ancestor_record_offset":260,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12312,"parent_record_offset":12276,"ancestor_record_offset":260,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1196,"parent_record_offset":1076,"ancestor_record_offset":260,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":4764,"parent_record_offset":4728,"ancestor_record_offset":260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":8928,"parent_record_offset":8900,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12228,"parent_record_offset":11040,"ancestor_record_offset":260,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":373,"record_offset":22984,"parent_record_offset":22960,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":373,"record_offset":23032,"parent_record_offset":23008,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":373,"record_offset":15724,"parent_record_offset":15288,"ancestor_record_offset":260,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":373,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":260,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=12
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":373,"record_offset":1044,"parent_record_offset":1016,"ancestor_record_offset":260,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":373,"record_offset":21168,"parent_record_offset":20960,"ancestor_record_offset":260,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":373,"record_offset":4880,"parent_record_offset":4836,"ancestor_record_offset":260,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":373,"record_offset":4908,"parent_record_offset":4880,"ancestor_record_offset":260,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":373,"record_offset":8780,"parent_record_offset":8704,"ancestor_record_offset":260,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":373,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":373,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":373,"record_offset":15596,"parent_record_offset":15564,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":373,"record_offset":724,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":373,"record_offset":23440,"parent_record_offset":23412,"ancestor_record_offset":260,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":373,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":260,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":373,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":260,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":373,"record_offset":6704,"parent_record_offset":6668,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":373,"record_offset":8432,"parent_record_offset":8248,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":373,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":373,"record_offset":17020,"parent_record_offset":16992,"ancestor_record_offset":260,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":373,"record_offset":15564,"parent_record_offset":15532,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":373,"record_offset":15532,"parent_record_offset":15288,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":373,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":373,"record_offset":21084,"parent_record_offset":20960,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":373,"record_offset":8804,"parent_record_offset":8780,"ancestor_record_offset":260,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":373,"record_offset":6016,"parent_record_offset":5988,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":373,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=38
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":373,"record_offset":6264,"parent_record_offset":6232,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":373,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":260,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7348,"parent_record_offset":7324,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7376,"parent_record_offset":7324,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":16960,"parent_record_offset":16936,"ancestor_record_offset":260,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":373,"record_offset":8752,"parent_record_offset":8704,"ancestor_record_offset":260,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":373,"record_offset":8704,"parent_record_offset":8676,"ancestor_record_offset":260,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":373,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":373,"record_offset":8292,"parent_record_offset":8248,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":373,"record_offset":5928,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":373,"record_offset":5716,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":373,"record_offset":5960,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":373,"record_offset":23536,"parent_record_offset":23384,"ancestor_record_offset":260,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":373,"record_offset":16800,"parent_record_offset":16512,"ancestor_record_offset":260,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":373,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":260,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=78
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":373,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":373,"record_offset":2340,"parent_record_offset":2316,"ancestor_record_offset":260,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":373,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":260,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":373,"record_offset":1596,"parent_record_offset":1480,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":373,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":260,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":373,"record_offset":1540,"parent_record_offset":1480,"ancestor_record_offset":260,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":373,"record_offset":1512,"parent_record_offset":1480,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":260,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":5020,"parent_record_offset":4992,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":15636,"parent_record_offset":15288,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":373,"record_offset":5840,"parent_record_offset":5752,"ancestor_record_offset":260,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":373,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":260,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":373,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":373,"record_offset":808,"parent_record_offset":780,"ancestor_record_offset":260,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":373,"record_offset":836,"parent_record_offset":808,"ancestor_record_offset":260,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":1224,"parent_record_offset":1076,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":373,"record_offset":5900,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":8960,"parent_record_offset":8928,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":18264,"parent_record_offset":18236,"ancestor_record_offset":260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":18236,"parent_record_offset":18208,"ancestor_record_offset":260,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":373,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":373,"record_offset":19148,"parent_record_offset":19084,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":373,"record_offset":15192,"parent_record_offset":15108,"ancestor_record_offset":260,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":373,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":373,"record_offset":15108,"parent_record_offset":15072,"ancestor_record_offset":260,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\specialize.rs
//     evidence={"module":373,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":260,"inlinee":12291,"source_line":18,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":373,"record_offset":752,"parent_record_offset":724,"ancestor_record_offset":260,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":6452,"parent_record_offset":6416,"ancestor_record_offset":260,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":5784,"parent_record_offset":5752,"ancestor_record_offset":260,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":260,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":260,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":3880,"parent_record_offset":3820,"ancestor_record_offset":260,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":3912,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":373,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":260,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":373,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":260,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":373,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":260,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":373,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":260,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":9836,"parent_record_offset":9804,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":11456,"parent_record_offset":11040,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9608,"parent_record_offset":9576,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9576,"parent_record_offset":9544,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":260,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":260,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":12060,"parent_record_offset":11040,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":12004,"parent_record_offset":11040,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":373,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":260,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":260,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":260,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":260,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":23276,"parent_record_offset":23252,"ancestor_record_offset":260,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":2380,"parent_record_offset":2156,"ancestor_record_offset":260,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":23616,"parent_record_offset":23584,"ancestor_record_offset":260,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":373,"record_offset":1708,"parent_record_offset":1676,"ancestor_record_offset":260,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":260,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2268,"parent_record_offset":2156,"ancestor_record_offset":260,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":260,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":24028,"parent_record_offset":23688,"ancestor_record_offset":260,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2588,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23384,"parent_record_offset":23324,"ancestor_record_offset":260,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23688,"parent_record_offset":23648,"ancestor_record_offset":260,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":260,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":260,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":1736,"parent_record_offset":1676,"ancestor_record_offset":260,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":260,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23356,"parent_record_offset":23324,"ancestor_record_offset":260,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":1784,"parent_record_offset":1760,"ancestor_record_offset":260,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":260,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":373,"record_offset":2036,"parent_record_offset":2004,"ancestor_record_offset":260,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":373,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":260,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":373,"record_offset":2124,"parent_record_offset":2004,"ancestor_record_offset":260,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":373,"record_offset":2004,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":1836,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":23648,"parent_record_offset":23616,"ancestor_record_offset":260,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":373,"record_offset":2156,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":373,"record_offset":1864,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":373,"record_offset":23412,"parent_record_offset":23384,"ancestor_record_offset":260,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":373,"record_offset":1948,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":373,"record_offset":2496,"parent_record_offset":1784,"ancestor_record_offset":260,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":373,"record_offset":23468,"parent_record_offset":23412,"ancestor_record_offset":260,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":373,"record_offset":7084,"parent_record_offset":7060,"ancestor_record_offset":260,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":373,"record_offset":7264,"parent_record_offset":7208,"ancestor_record_offset":260,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":373,"record_offset":5752,"parent_record_offset":5648,"ancestor_record_offset":260,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":373,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":260,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":373,"record_offset":9640,"parent_record_offset":9608,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":373,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":260,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":11128,"parent_record_offset":11040,"ancestor_record_offset":260,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":260,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":10340,"parent_record_offset":10312,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":9036,"parent_record_offset":8900,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":7408,"parent_record_offset":7208,"ancestor_record_offset":260,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":373,"record_offset":6732,"parent_record_offset":6668,"ancestor_record_offset":260,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":373,"record_offset":9172,"parent_record_offset":8900,"ancestor_record_offset":260,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":373,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":260,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEAB9
// 首条位置: {"module":1264,"inlinee":60089}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1264,"record_offset":8712,"parent_record_offset":8172,"ancestor_record_offset":8172,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1264,"record_offset":8200,"parent_record_offset":8172,"ancestor_record_offset":8172,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1264,"record_offset":8228,"parent_record_offset":8172,"ancestor_record_offset":8172,"inlinee":21574,"source_line":55,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1264,"record_offset":8852,"parent_record_offset":8828,"ancestor_record_offset":8172,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1264,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8172,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1264,"record_offset":8416,"parent_record_offset":8388,"ancestor_record_offset":8172,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1264,"record_offset":8944,"parent_record_offset":8768,"ancestor_record_offset":8172,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":8172,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1264,"record_offset":8356,"parent_record_offset":8276,"ancestor_record_offset":8172,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1264,"record_offset":8828,"parent_record_offset":8804,"ancestor_record_offset":8172,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1264,"record_offset":8804,"parent_record_offset":8768,"ancestor_record_offset":8172,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1264,"record_offset":8276,"parent_record_offset":8228,"ancestor_record_offset":8172,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1264,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":8172,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1264,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":8172,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1264,"record_offset":8388,"parent_record_offset":8228,"ancestor_record_offset":8172,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1264,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":8172,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1264,"record_offset":8740,"parent_record_offset":8712,"ancestor_record_offset":8172,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01264.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::update_phys
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5455
// 首条位置: {"module":373,"inlinee":21589}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":21280,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":21052,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":373,"record_offset":21168,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":373,"record_offset":21084,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":21648,"parent_record_offset":20960,"ancestor_record_offset":20960,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":20960,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":20960,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":22084,"parent_record_offset":22060,"ancestor_record_offset":20960,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":21348,"parent_record_offset":21312,"ancestor_record_offset":20960,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":21380,"parent_record_offset":21348,"ancestor_record_offset":20960,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":373,"record_offset":22060,"parent_record_offset":21968,"ancestor_record_offset":20960,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":373,"record_offset":21312,"parent_record_offset":21280,"ancestor_record_offset":20960,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":21996,"parent_record_offset":21968,"ancestor_record_offset":20960,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":20960,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":21780,"parent_record_offset":21744,"ancestor_record_offset":20960,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":373,"record_offset":21744,"parent_record_offset":21720,"ancestor_record_offset":20960,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":21108,"parent_record_offset":21084,"ancestor_record_offset":20960,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":373,"record_offset":21192,"parent_record_offset":21168,"ancestor_record_offset":20960,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":373,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":20960,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":373,"record_offset":22024,"parent_record_offset":21996,"ancestor_record_offset":20960,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":21500,"parent_record_offset":21312,"ancestor_record_offset":20960,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":373,"record_offset":21812,"parent_record_offset":21780,"ancestor_record_offset":20960,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":373,"record_offset":21456,"parent_record_offset":21428,"ancestor_record_offset":20960,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":21840,"parent_record_offset":21780,"ancestor_record_offset":20960,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":373,"record_offset":21872,"parent_record_offset":21744,"ancestor_record_offset":20960,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::update_phys::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8EA
// 首条位置: {"module":1226,"inlinee":59626}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::add_colision_geo
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5450
// 首条位置: {"module":373,"inlinee":21584}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12228,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":373,"record_offset":11456,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11976,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":12060,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":12004,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":11128,"parent_record_offset":11040,"ancestor_record_offset":11040,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":11040,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12364,"parent_record_offset":12336,"ancestor_record_offset":11040,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12504,"parent_record_offset":12464,"ancestor_record_offset":11040,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":373,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":11040,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":11040,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12464,"parent_record_offset":12276,"ancestor_record_offset":11040,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12436,"parent_record_offset":12276,"ancestor_record_offset":11040,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":12312,"parent_record_offset":12276,"ancestor_record_offset":11040,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":373,"record_offset":11548,"parent_record_offset":11516,"ancestor_record_offset":11040,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":373,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":11040,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":11412,"parent_record_offset":11284,"ancestor_record_offset":11040,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":12084,"parent_record_offset":12060,"ancestor_record_offset":11040,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":11040,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":373,"record_offset":12796,"parent_record_offset":12596,"ancestor_record_offset":11040,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":11576,"parent_record_offset":11548,"ancestor_record_offset":11040,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":373,"record_offset":11604,"parent_record_offset":11576,"ancestor_record_offset":11040,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":11040,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11312,"parent_record_offset":11284,"ancestor_record_offset":11040,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11284,"parent_record_offset":11260,"ancestor_record_offset":11040,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11260,"parent_record_offset":11236,"ancestor_record_offset":11040,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":11040,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":373,"record_offset":11376,"parent_record_offset":11344,"ancestor_record_offset":11040,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":373,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":11040,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":373,"record_offset":11236,"parent_record_offset":11128,"ancestor_record_offset":11040,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/00373.jsonl.gz"}
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/platform-stairs-findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::add_colision_geo
// 信息 ID: f2801fb947e5f72526d7be8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=11040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::construct_platform_stairs
// 信息 ID: 48697a1652dbe449118f3b47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 83051c333104938caadef7fe: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> @ 0x8192b -> 0x4ef4b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)> > @ 0xd49ed -> 0x4ef4b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure$0
// 信息 ID: ff13bb856caade6ed0bda6f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::construct_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >; module=1264, inline_record=8172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::update_phys
// 信息 ID: 905a2aa4d7e32079f1bd83b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=20960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::update_phys::closure$0
// 信息 ID: 859f8be7eb6404881b3501f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::update_phys", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::update_phys::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u32>,system_wall_constructor::construct_platform_stairs::update_phys::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::slice::iter::Chunks<u32>,system_wall_constructor::construct_platform_stairs::update_phys::closure_env$0> >; module=1226, inline_record=60084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
