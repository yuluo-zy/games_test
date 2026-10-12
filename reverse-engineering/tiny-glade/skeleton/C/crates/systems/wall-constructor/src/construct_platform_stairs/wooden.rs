// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 61758fc5d24381e83bd8f1dba24422c50edb172f275ed652538220d4c816f0d9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wooden.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1459, 1472, 1474, 1482, 1500, 1525, 3141, 3144, 3159, 3162, 4370, 4371, 4375
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1472,"record_offset":280,"rva":21080464,"va":5389789584}
// 编译实例 RVAs: 0x141a990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14119869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8438c28ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389798552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=25
//     evidence={"assembly":"\tcallq\t-14113191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e859a628ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389791890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21076352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e88066befe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389798539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20563012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8bc3bc6fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5389803535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20253281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e89ff5cafe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389795276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6469716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8ac479dff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389794767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8f69ffeff","size":5,"soft_fail":false,"target_va":5389703456,"va":5389793573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e88fa7feff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389792844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e862e1ffff","size":5,"soft_fail":false,"target_va":5389786032,"va":5389793865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t324588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8ecf30400","size":5,"soft_fail":false,"target_va":5390124608,"va":5389800015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424dcd40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17566909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8bd0c0c01","size":5,"soft_fail":false,"target_va":5407362368,"va":5389795454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424ddbb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17566776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8380c0c01","size":5,"soft_fail":false,"target_va":5407366064,"va":5389799283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26559871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e87f459501","size":5,"soft_fail":false,"target_va":5416351056,"va":5389791180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26561100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e84c4a9501","size":5,"soft_fail":false,"target_va":5416353040,"va":5389791935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8bc08a001","size":5,"soft_fail":false,"target_va":5417057056,"va":5389791839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27263831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e85703a001","size":5,"soft_fail":false,"target_va":5417057792,"va":5389793956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8e709a001","size":5,"soft_fail":false,"target_va":5417057840,"va":5389792324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: wooden.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27265815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8170ba001","size":5,"soft_fail":false,"target_va":5417057984,"va":5389792164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26821697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e841449901","size":5,"soft_fail":false,"target_va":5416620794,"va":5389799092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26821798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8a6449901","size":5,"soft_fail":false,"target_va":5416621600,"va":5389799797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t26822632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8e8479901","size":5,"soft_fail":false,"target_va":5416622208,"va":5389799571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t26821721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e859449901","size":5,"soft_fail":false,"target_va":5416621762,"va":5389800036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26822745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e859489901","size":5,"soft_fail":false,"target_va":5416622240,"va":5389799490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<array$<array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>,4> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8fe100000","size":5,"soft_fail":false,"target_va":5389803648,"va":5389799293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26822376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8e8469901","size":5,"soft_fail":false,"target_va":5416622320,"va":5389799939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t350525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e83d590500","size":5,"soft_fail":false,"target_va":5390150736,"va":5389800206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8559113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8f7657dff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389793716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10580568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8a88d5eff","size":5,"soft_fail":false,"target_va":5379218768,"va":5389799331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10575266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e85ea25eff","size":5,"soft_fail":false,"target_va":5379215376,"va":5389790637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10196161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e83f6b64ff","size":5,"soft_fail":false,"target_va":5379597392,"va":5389793548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::add_colision_geo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12759199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8614f3dff","size":5,"soft_fail":false,"target_va":5377035056,"va":5389794250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::emit_wooden_stair_layout
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17561122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e822f60b01","size":5,"soft_fail":false,"target_va":5407356528,"va":5389795401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::wooden_stair_layout
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17560183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e877f20b01","size":5,"soft_fail":false,"target_va":5407355280,"va":5389795092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::wooden_corners::add_ramp_collision
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17565920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8e0080c01","size":5,"soft_fail":false,"target_va":5407364352,"va":5389798427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6488088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8e8ff9cff","size":5,"soft_fail":false,"target_va":5383305360,"va":5389793443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6473215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8013a9dff","size":5,"soft_fail":false,"target_va":5383316912,"va":5389790122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6483004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8c4139dff","size":5,"soft_fail":false,"target_va":5383313312,"va":5389796311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6473070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8923a9dff","size":5,"soft_fail":false,"target_va":5383323216,"va":5389796281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6471895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8293f9dff","size":5,"soft_fail":false,"target_va":5383317936,"va":5389789826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6475495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e819319dff","size":5,"soft_fail":false,"target_va":5383314512,"va":5389790002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6466723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e85d539dff","size":5,"soft_fail":false,"target_va":5383323424,"va":5389790142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6518207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8418a9cff","size":5,"soft_fail":false,"target_va":5383274080,"va":5389792282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6494265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389789584,"raw_hex":"e8c7e79cff","size":5,"soft_fail":false,"target_va":5383297984,"va":5389792244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":26596,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::build_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":13260,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79538,"source_line":342,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":464,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79531,"source_line":83,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9196,"parent_record_offset":9016,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3276,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2780,"parent_record_offset":2696,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":280,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2860,"parent_record_offset":2820,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2820,"parent_record_offset":2780,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9016,"parent_record_offset":8984,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9052,"parent_record_offset":9016,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":4608,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":8884,"parent_record_offset":8816,"ancestor_record_offset":280,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":4644,"parent_record_offset":4608,"ancestor_record_offset":280,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3480,"parent_record_offset":3440,"ancestor_record_offset":280,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2696,"parent_record_offset":2600,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8856,"parent_record_offset":8816,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3820,"parent_record_offset":3480,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":868,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2560,"parent_record_offset":2388,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2600,"parent_record_offset":2560,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3528,"parent_record_offset":3480,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8928,"parent_record_offset":8884,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8784,"parent_record_offset":8752,"ancestor_record_offset":280,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":16788,"parent_record_offset":16712,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8560,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":5648,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3408,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":280,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":280,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":4816,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1472,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":280,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1472,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1472,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":280,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":5592,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":26753,"source_line":109,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1192,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":6828,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":7088,"parent_record_offset":7052,"ancestor_record_offset":280,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":280,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":280,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7116,"parent_record_offset":7088,"ancestor_record_offset":280,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":280,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7336,"parent_record_offset":7300,"ancestor_record_offset":280,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":280,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":280,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":13776,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":5968,"parent_record_offset":5932,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":7728,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1472,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":1804,"parent_record_offset":1764,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":13932,"parent_record_offset":13896,"ancestor_record_offset":280,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3024,"parent_record_offset":2388,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3068,"parent_record_offset":2388,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":280,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":280,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1472,"record_offset":4720,"parent_record_offset":4680,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1472,"record_offset":9156,"parent_record_offset":9100,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1472,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1472,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1472,"record_offset":26644,"parent_record_offset":26620,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1472,"record_offset":26736,"parent_record_offset":26668,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6964,"parent_record_offset":6884,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7024,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":280,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":280,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":6000,"parent_record_offset":5932,"ancestor_record_offset":280,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2968,"parent_record_offset":2388,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1472,"record_offset":26620,"parent_record_offset":26596,"ancestor_record_offset":280,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":1472,"record_offset":26788,"parent_record_offset":26736,"ancestor_record_offset":280,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":280,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1724,"parent_record_offset":1684,"ancestor_record_offset":280,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":6884,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":14608,"parent_record_offset":14584,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1472,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":280,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":8652,"parent_record_offset":8616,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9128,"parent_record_offset":9100,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":8504,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=25
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":280,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":280,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":280,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16032,"parent_record_offset":16008,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16008,"parent_record_offset":15976,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1472,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":280,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1472,"record_offset":18108,"parent_record_offset":16476,"ancestor_record_offset":280,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":14560,"parent_record_offset":14536,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":1092,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":9324,"parent_record_offset":8884,"ancestor_record_offset":280,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":280,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":3112,"parent_record_offset":2388,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1472,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":14480,"parent_record_offset":14396,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":4892,"parent_record_offset":4816,"ancestor_record_offset":280,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":280,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":2916,"parent_record_offset":2600,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1472,"record_offset":19548,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1472,"record_offset":19572,"parent_record_offset":19548,"ancestor_record_offset":280,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":1064,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":1128,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":5164,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8684,"parent_record_offset":8616,"ancestor_record_offset":280,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":280,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16844,"parent_record_offset":16816,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":280,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1472,"record_offset":26668,"parent_record_offset":26644,"ancestor_record_offset":280,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1472,"record_offset":1008,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":896,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1472,"record_offset":4680,"parent_record_offset":4644,"ancestor_record_offset":280,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":280,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":280,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2208,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2288,"parent_record_offset":2232,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18912,"parent_record_offset":18876,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18876,"parent_record_offset":18844,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18844,"parent_record_offset":18324,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":19088,"parent_record_offset":18324,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6148,"parent_record_offset":6064,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8212,"parent_record_offset":8176,"ancestor_record_offset":280,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8176,"parent_record_offset":8148,"ancestor_record_offset":280,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":280,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17044,"parent_record_offset":17008,"ancestor_record_offset":280,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":280,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":280,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8088,"parent_record_offset":7844,"ancestor_record_offset":280,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8324,"parent_record_offset":7844,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8412,"parent_record_offset":7844,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":7996,"parent_record_offset":7844,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1472,"record_offset":14512,"parent_record_offset":14352,"ancestor_record_offset":280,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1472,"record_offset":14352,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1472,"record_offset":6292,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7672,"parent_record_offset":6752,"ancestor_record_offset":280,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":5820,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":2320,"parent_record_offset":2288,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::WoodenStairDebugParams::style
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1472,"record_offset":8532,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":26140,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":1472,"record_offset":8752,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79550,"source_line":74,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds::wooden_profile
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":7844,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79547,"source_line":324,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":15948,"parent_record_offset":15920,"ancestor_record_offset":280,"inlinee":79542,"source_line":370,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":16664,"parent_record_offset":16616,"ancestor_record_offset":280,"inlinee":79435,"source_line":371,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":18324,"parent_record_offset":18296,"ancestor_record_offset":280,"inlinee":79543,"source_line":372,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$3
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":19312,"parent_record_offset":19284,"ancestor_record_offset":280,"inlinee":79544,"source_line":379,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":280,"inlinee":79533,"source_line":101,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$3
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":6064,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79548,"source_line":135,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":26760,"parent_record_offset":26736,"ancestor_record_offset":280,"inlinee":79552,"source_line":76,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1552,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79534,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1764,"parent_record_offset":1724,"ancestor_record_offset":280,"inlinee":79536,"source_line":230,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::sample_profile_bounds
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":6752,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79545,"source_line":288,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::split_side_intervals
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":2388,"parent_record_offset":464,"ancestor_record_offset":280,"inlinee":79537,"source_line":234,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1472,"record_offset":13984,"parent_record_offset":13960,"ancestor_record_offset":280,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16712,"parent_record_offset":16664,"ancestor_record_offset":280,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18368,"parent_record_offset":18324,"ancestor_record_offset":280,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18400,"parent_record_offset":18368,"ancestor_record_offset":280,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16760,"parent_record_offset":16712,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16884,"parent_record_offset":16712,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":15976,"parent_record_offset":15948,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":13868,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":13896,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":13960,"parent_record_offset":13896,"ancestor_record_offset":280,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14744,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14240,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":79540,"source_line":9,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":17008,"parent_record_offset":16712,"ancestor_record_offset":280,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":18556,"parent_record_offset":18368,"ancestor_record_offset":280,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136C0
// 首条位置: {"module":1472,"inlinee":79552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136AB
// 首条位置: {"module":1472,"inlinee":79531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":4608,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":868,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":4952,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8560,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":5648,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3408,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":4816,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::array::from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":5592,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":26753,"source_line":109,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1192,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":5192,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":5904,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":8504,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":1092,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":5220,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":1064,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":1128,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":5164,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1472,"record_offset":1008,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":896,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2208,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1472,"record_offset":6292,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":5820,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::WoodenStairDebugParams::style
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1472,"record_offset":8532,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":26140,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::emit_wooden_stair_pieces::impl$4::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/emit_wooden_stair_pieces.rs
//     evidence={"module":1472,"record_offset":8752,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79550,"source_line":74,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds::wooden_profile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":7844,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79547,"source_line":324,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$3
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":6064,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79548,"source_line":135,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1552,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79534,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::sample_profile_bounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":6752,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79545,"source_line":288,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::split_side_intervals
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":2388,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":79537,"source_line":234,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":464,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":464,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":464,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":464,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":464,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":464,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":9196,"parent_record_offset":9016,"ancestor_record_offset":464,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3276,"parent_record_offset":3184,"ancestor_record_offset":464,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2780,"parent_record_offset":2696,"ancestor_record_offset":464,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":464,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8956,"parent_record_offset":8928,"ancestor_record_offset":464,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":464,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":464,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":464,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":464,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2860,"parent_record_offset":2820,"ancestor_record_offset":464,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2820,"parent_record_offset":2780,"ancestor_record_offset":464,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":464,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9016,"parent_record_offset":8984,"ancestor_record_offset":464,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":8984,"parent_record_offset":8956,"ancestor_record_offset":464,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":464,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":9052,"parent_record_offset":9016,"ancestor_record_offset":464,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":8884,"parent_record_offset":8816,"ancestor_record_offset":464,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1472,"record_offset":4644,"parent_record_offset":4608,"ancestor_record_offset":464,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3480,"parent_record_offset":3440,"ancestor_record_offset":464,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2696,"parent_record_offset":2600,"ancestor_record_offset":464,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8856,"parent_record_offset":8816,"ancestor_record_offset":464,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3820,"parent_record_offset":3480,"ancestor_record_offset":464,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2532,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2560,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2600,"parent_record_offset":2560,"ancestor_record_offset":464,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":3528,"parent_record_offset":3480,"ancestor_record_offset":464,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8928,"parent_record_offset":8884,"ancestor_record_offset":464,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8784,"parent_record_offset":8752,"ancestor_record_offset":464,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":464,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":464,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1472,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":464,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1472,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":464,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1472,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":464,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1472,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":464,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":464,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":6828,"parent_record_offset":6752,"ancestor_record_offset":464,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":7088,"parent_record_offset":7052,"ancestor_record_offset":464,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":464,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":464,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7116,"parent_record_offset":7088,"ancestor_record_offset":464,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":464,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7336,"parent_record_offset":7300,"ancestor_record_offset":464,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":464,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":464,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":464,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":5968,"parent_record_offset":5932,"ancestor_record_offset":464,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":7728,"parent_record_offset":6752,"ancestor_record_offset":464,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1472,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":464,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":1804,"parent_record_offset":1764,"ancestor_record_offset":464,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3024,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3068,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":464,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":464,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":464,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1472,"record_offset":4720,"parent_record_offset":4680,"ancestor_record_offset":464,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1472,"record_offset":9156,"parent_record_offset":9100,"ancestor_record_offset":464,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6964,"parent_record_offset":6884,"ancestor_record_offset":464,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7024,"parent_record_offset":6752,"ancestor_record_offset":464,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":464,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":464,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":6000,"parent_record_offset":5932,"ancestor_record_offset":464,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":464,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2968,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":464,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1724,"parent_record_offset":1684,"ancestor_record_offset":464,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":6884,"parent_record_offset":6752,"ancestor_record_offset":464,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1472,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":464,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":464,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":8652,"parent_record_offset":8616,"ancestor_record_offset":464,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":9128,"parent_record_offset":9100,"ancestor_record_offset":464,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":464,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":464,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":464,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":464,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":464,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":464,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":464,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":464,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1472,"record_offset":1316,"parent_record_offset":1288,"ancestor_record_offset":464,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":464,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":9324,"parent_record_offset":8884,"ancestor_record_offset":464,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":464,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":9348,"parent_record_offset":9324,"ancestor_record_offset":464,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":3112,"parent_record_offset":2388,"ancestor_record_offset":464,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1472,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":464,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":4892,"parent_record_offset":4816,"ancestor_record_offset":464,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":464,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":464,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":2916,"parent_record_offset":2600,"ancestor_record_offset":464,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":464,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8684,"parent_record_offset":8616,"ancestor_record_offset":464,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":464,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":464,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":464,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1472,"record_offset":4680,"parent_record_offset":4644,"ancestor_record_offset":464,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":464,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":464,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2288,"parent_record_offset":2232,"ancestor_record_offset":464,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":464,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6148,"parent_record_offset":6064,"ancestor_record_offset":464,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":464,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8212,"parent_record_offset":8176,"ancestor_record_offset":464,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8176,"parent_record_offset":8148,"ancestor_record_offset":464,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":464,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":464,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8088,"parent_record_offset":7844,"ancestor_record_offset":464,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8324,"parent_record_offset":7844,"ancestor_record_offset":464,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8412,"parent_record_offset":7844,"ancestor_record_offset":464,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":7996,"parent_record_offset":7844,"ancestor_record_offset":464,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7672,"parent_record_offset":6752,"ancestor_record_offset":464,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":2320,"parent_record_offset":2288,"ancestor_record_offset":464,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1344,"parent_record_offset":1316,"ancestor_record_offset":464,"inlinee":79533,"source_line":101,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1764,"parent_record_offset":1724,"ancestor_record_offset":464,"inlinee":79536,"source_line":230,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136AD
// 首条位置: {"module":1472,"inlinee":79533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":1344,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1344,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=41
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1380A
// 首条位置: {"module":1500,"inlinee":79882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::then
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1500,"record_offset":14804,"parent_record_offset":14752,"ancestor_record_offset":14752,"inlinee":22473,"source_line":613,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1500,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14752,"inlinee":9495,"source_line":1440,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136BC
// 首条位置: {"module":1472,"inlinee":79548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":6204,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":6148,"parent_record_offset":6064,"ancestor_record_offset":6064,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1472,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":6064,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":6064,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":6064,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1472,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":6064,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBFB
// 首条位置: {"module":3141,"inlinee":117755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136AE
// 首条位置: {"module":1472,"inlinee":79534}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1588,"parent_record_offset":1552,"ancestor_record_offset":1552,"inlinee":7940,"source_line":2937,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1552,"inlinee":5092,"source_line":257,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1552,"inlinee":5093,"source_line":206,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1552,"inlinee":5097,"source_line":215,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":1804,"parent_record_offset":1764,"ancestor_record_offset":1552,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":1724,"parent_record_offset":1684,"ancestor_record_offset":1552,"inlinee":5484,"source_line":2944,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1552,"inlinee":5095,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":1764,"parent_record_offset":1724,"ancestor_record_offset":1552,"inlinee":79536,"source_line":230,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B0
// 首条位置: {"module":1472,"inlinee":79536}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":1804,"parent_record_offset":1764,"ancestor_record_offset":1764,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::split_side_intervals
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B1
// 首条位置: {"module":1472,"inlinee":79537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2532,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2560,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3024,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":3068,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2968,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":3112,"parent_record_offset":2388,"ancestor_record_offset":2388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2388,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":2388,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":2388,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":3276,"parent_record_offset":3184,"ancestor_record_offset":2388,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2780,"parent_record_offset":2696,"ancestor_record_offset":2388,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3212,"parent_record_offset":3184,"ancestor_record_offset":2388,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2388,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2860,"parent_record_offset":2820,"ancestor_record_offset":2388,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":2820,"parent_record_offset":2780,"ancestor_record_offset":2388,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2388,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2696,"parent_record_offset":2600,"ancestor_record_offset":2388,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":2600,"parent_record_offset":2560,"ancestor_record_offset":2388,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1472,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2388,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":3244,"parent_record_offset":3212,"ancestor_record_offset":2388,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":2916,"parent_record_offset":2600,"ancestor_record_offset":2388,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::sample_profile_bounds
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 288
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B9
// 首条位置: {"module":1472,"inlinee":79545}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":6828,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":7728,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7756,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7644,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7024,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":6884,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":7468,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":7556,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7672,"parent_record_offset":6752,"ancestor_record_offset":6752,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":6752,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":7088,"parent_record_offset":7052,"ancestor_record_offset":6752,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":6752,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7116,"parent_record_offset":7088,"ancestor_record_offset":6752,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":7336,"parent_record_offset":7300,"ancestor_record_offset":6752,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6752,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6964,"parent_record_offset":6884,"ancestor_record_offset":6752,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6752,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":6752,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1472,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":6752,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":6752,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":7584,"parent_record_offset":7556,"ancestor_record_offset":6752,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":7496,"parent_record_offset":7468,"ancestor_record_offset":6752,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":6752,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":6752,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":6752,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":6752,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":6752,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds::wooden_profile
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 324
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136BB
// 首条位置: {"module":1472,"inlinee":79547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":7884,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":7940,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8088,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8324,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8412,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":7996,"parent_record_offset":7844,"ancestor_record_offset":7844,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":7844,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":7844,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7844,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":8028,"parent_record_offset":7996,"ancestor_record_offset":7844,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8212,"parent_record_offset":8176,"ancestor_record_offset":7844,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8176,"parent_record_offset":8148,"ancestor_record_offset":7844,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":7844,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7844,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":8248,"parent_record_offset":8212,"ancestor_record_offset":7844,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 342
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B2
// 首条位置: {"module":1472,"inlinee":79538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":25208,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":15780,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":13776,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":25180,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":10423,"source_line":2456,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":13832,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":19604,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1472,"record_offset":19548,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":11666,"source_line":646,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":13804,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":14268,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":14020,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1472,"record_offset":13664,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":13552,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":23552,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":23524,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1472,"record_offset":14352,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":13720,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::WoodenStairDebugParams::style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1472,"record_offset":25152,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":26140,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds::wooden_profile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":23768,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":79547,"source_line":324,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::sample_profile_bounds
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":20856,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":79545,"source_line":288,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":22568,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":23092,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":22420,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":13868,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":13896,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14744,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14240,"parent_record_offset":13260,"ancestor_record_offset":13260,"inlinee":79540,"source_line":9,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":19840,"parent_record_offset":19816,"ancestor_record_offset":13260,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":19864,"parent_record_offset":19840,"ancestor_record_offset":13260,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":19888,"parent_record_offset":19864,"ancestor_record_offset":13260,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1472,"record_offset":19816,"parent_record_offset":19724,"ancestor_record_offset":13260,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":19752,"parent_record_offset":19724,"ancestor_record_offset":13260,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":13260,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1472,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":13260,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":16788,"parent_record_offset":16712,"ancestor_record_offset":13260,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1472,"record_offset":15892,"parent_record_offset":15864,"ancestor_record_offset":13260,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":15808,"parent_record_offset":15780,"ancestor_record_offset":13260,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$1::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":20928,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":15318,"source_line":54,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1472,"record_offset":21188,"parent_record_offset":21152,"ancestor_record_offset":13260,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::new_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":13260,"inlinee":15319,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":21216,"parent_record_offset":21188,"ancestor_record_offset":13260,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1472,"record_offset":21432,"parent_record_offset":21396,"ancestor_record_offset":13260,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":15836,"parent_record_offset":15808,"ancestor_record_offset":13260,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1472,"record_offset":15864,"parent_record_offset":15836,"ancestor_record_offset":13260,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":13260,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":13260,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":21824,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":13932,"parent_record_offset":13896,"ancestor_record_offset":13260,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":21852,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":21740,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":13260,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":13260,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":13260,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1472,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":13260,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1472,"record_offset":14584,"parent_record_offset":14560,"ancestor_record_offset":13260,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":21064,"parent_record_offset":20984,"ancestor_record_offset":13260,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":21124,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":21088,"parent_record_offset":21064,"ancestor_record_offset":13260,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1472,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":13260,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":20984,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1472,"record_offset":14608,"parent_record_offset":14584,"ancestor_record_offset":13260,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1472,"record_offset":21456,"parent_record_offset":21432,"ancestor_record_offset":13260,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":25300,"parent_record_offset":25264,"ancestor_record_offset":13260,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":13260,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1472,"record_offset":19784,"parent_record_offset":19752,"ancestor_record_offset":13260,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14912,"parent_record_offset":14860,"ancestor_record_offset":13260,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":13260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":14824,"parent_record_offset":14800,"ancestor_record_offset":13260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":21272,"parent_record_offset":21244,"ancestor_record_offset":13260,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1472,"record_offset":21244,"parent_record_offset":21216,"ancestor_record_offset":13260,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16032,"parent_record_offset":16008,"ancestor_record_offset":13260,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16008,"parent_record_offset":15976,"ancestor_record_offset":13260,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1472,"record_offset":15920,"parent_record_offset":15892,"ancestor_record_offset":13260,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$3::from_output
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1472,"record_offset":18108,"parent_record_offset":16476,"ancestor_record_offset":13260,"inlinee":26507,"source_line":453,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":21396,"parent_record_offset":21368,"ancestor_record_offset":13260,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1472,"record_offset":14560,"parent_record_offset":14536,"ancestor_record_offset":13260,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1472,"record_offset":21480,"parent_record_offset":21456,"ancestor_record_offset":13260,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":13260,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1472,"record_offset":13624,"parent_record_offset":13600,"ancestor_record_offset":13260,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":14480,"parent_record_offset":14396,"ancestor_record_offset":13260,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1472,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":13260,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1472,"record_offset":21504,"parent_record_offset":21480,"ancestor_record_offset":13260,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1472,"record_offset":19572,"parent_record_offset":19548,"ancestor_record_offset":13260,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1472,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":13260,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":25332,"parent_record_offset":25264,"ancestor_record_offset":13260,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":13260,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":25236,"parent_record_offset":25208,"ancestor_record_offset":13260,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16844,"parent_record_offset":16816,"ancestor_record_offset":13260,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":25264,"parent_record_offset":25236,"ancestor_record_offset":13260,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1472,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":13260,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1472,"record_offset":14396,"parent_record_offset":14352,"ancestor_record_offset":13260,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18948,"parent_record_offset":18912,"ancestor_record_offset":13260,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18912,"parent_record_offset":18876,"ancestor_record_offset":13260,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18876,"parent_record_offset":18844,"ancestor_record_offset":13260,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18844,"parent_record_offset":18324,"ancestor_record_offset":13260,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":19088,"parent_record_offset":18324,"ancestor_record_offset":13260,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":21564,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":21652,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18676,"parent_record_offset":18644,"ancestor_record_offset":13260,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18644,"parent_record_offset":18612,"ancestor_record_offset":13260,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18612,"parent_record_offset":18584,"ancestor_record_offset":13260,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17044,"parent_record_offset":17008,"ancestor_record_offset":13260,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":13260,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":24152,"parent_record_offset":24124,"ancestor_record_offset":13260,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":24124,"parent_record_offset":23768,"ancestor_record_offset":13260,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17080,"parent_record_offset":17044,"ancestor_record_offset":13260,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17240,"parent_record_offset":17044,"ancestor_record_offset":13260,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18428,"parent_record_offset":18400,"ancestor_record_offset":13260,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1472,"record_offset":14512,"parent_record_offset":14352,"ancestor_record_offset":13260,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1472,"record_offset":14800,"parent_record_offset":14744,"ancestor_record_offset":13260,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":21768,"parent_record_offset":20856,"ancestor_record_offset":13260,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":13260,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":15948,"parent_record_offset":15920,"ancestor_record_offset":13260,"inlinee":79542,"source_line":370,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":16664,"parent_record_offset":16616,"ancestor_record_offset":13260,"inlinee":79435,"source_line":371,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":18324,"parent_record_offset":18296,"ancestor_record_offset":13260,"inlinee":79543,"source_line":372,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$3
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":1472,"record_offset":19312,"parent_record_offset":19284,"ancestor_record_offset":13260,"inlinee":79544,"source_line":379,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1472,"record_offset":13984,"parent_record_offset":13960,"ancestor_record_offset":13260,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16712,"parent_record_offset":16664,"ancestor_record_offset":13260,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18368,"parent_record_offset":18324,"ancestor_record_offset":13260,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18400,"parent_record_offset":18368,"ancestor_record_offset":13260,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16760,"parent_record_offset":16712,"ancestor_record_offset":13260,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16884,"parent_record_offset":16712,"ancestor_record_offset":13260,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":15976,"parent_record_offset":15948,"ancestor_record_offset":13260,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":14944,"parent_record_offset":14744,"ancestor_record_offset":13260,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1472,"record_offset":13960,"parent_record_offset":13896,"ancestor_record_offset":13260,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":17008,"parent_record_offset":16712,"ancestor_record_offset":13260,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":18556,"parent_record_offset":18368,"ancestor_record_offset":13260,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 370
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B6
// 首条位置: {"module":1472,"inlinee":79542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1472,"record_offset":15976,"parent_record_offset":15948,"ancestor_record_offset":15948,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16056,"parent_record_offset":16032,"ancestor_record_offset":15948,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1472,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":15948,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16032,"parent_record_offset":16008,"ancestor_record_offset":15948,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1472,"record_offset":16008,"parent_record_offset":15976,"ancestor_record_offset":15948,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 371
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1364B
// 首条位置: {"module":1459,"inlinee":79435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16712,"parent_record_offset":16664,"ancestor_record_offset":16664,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1472,"record_offset":16788,"parent_record_offset":16712,"ancestor_record_offset":16664,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":17272,"parent_record_offset":17240,"ancestor_record_offset":16664,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":17120,"parent_record_offset":17080,"ancestor_record_offset":16664,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16816,"parent_record_offset":16788,"ancestor_record_offset":16664,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1472,"record_offset":16844,"parent_record_offset":16816,"ancestor_record_offset":16664,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17044,"parent_record_offset":17008,"ancestor_record_offset":16664,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17080,"parent_record_offset":17044,"ancestor_record_offset":16664,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":17240,"parent_record_offset":17044,"ancestor_record_offset":16664,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16760,"parent_record_offset":16712,"ancestor_record_offset":16664,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":16884,"parent_record_offset":16712,"ancestor_record_offset":16664,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":17008,"parent_record_offset":16712,"ancestor_record_offset":16664,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 372
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B7
// 首条位置: {"module":1472,"inlinee":79543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18844,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":19088,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18368,"parent_record_offset":18324,"ancestor_record_offset":18324,"inlinee":26602,"source_line":816,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1472,"record_offset":18732,"parent_record_offset":18708,"ancestor_record_offset":18324,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":18772,"parent_record_offset":18612,"ancestor_record_offset":18324,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1472,"record_offset":18464,"parent_record_offset":18428,"ancestor_record_offset":18324,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18948,"parent_record_offset":18912,"ancestor_record_offset":18324,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18912,"parent_record_offset":18876,"ancestor_record_offset":18324,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1472,"record_offset":18876,"parent_record_offset":18844,"ancestor_record_offset":18324,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18676,"parent_record_offset":18644,"ancestor_record_offset":18324,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18644,"parent_record_offset":18612,"ancestor_record_offset":18324,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18612,"parent_record_offset":18584,"ancestor_record_offset":18324,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":18324,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1472,"record_offset":18428,"parent_record_offset":18400,"ancestor_record_offset":18324,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1472,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":18324,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1472,"record_offset":18400,"parent_record_offset":18368,"ancestor_record_offset":18324,"inlinee":26603,"source_line":812,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1472,"record_offset":18556,"parent_record_offset":18368,"ancestor_record_offset":18324,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/01472.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 379
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136B8
// 首条位置: {"module":1472,"inlinee":79544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CBFA
// 首条位置: {"module":3141,"inlinee":117754}
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
// 完整结构及原始类型记录: wooden.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_platform_stairs::wooden::EndpointRun
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun>", "constructor": "core::option::Option", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::EndpointRun"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >", "constructor": "enum2$", "arguments": ["core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>", "constructor": "array$", "arguments": ["enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>,4>", "constructor": "array$", "arguments": ["array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>", "4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>,4> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<array$<enum2$<core::option::Option<system_wall_constructor::construct_platform_stairs::wooden::EndpointRun> >,2>,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_platform_stairs::wooden::SideInterval
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort8_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort8_stable", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insertion_sort_shift_left", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>,alloc::vec::Vec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>", "alloc::vec::Vec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["system_wall_constructor::construct_platform_stairs::wooden::SideInterval", "alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wooden.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wooden.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::ProfileBounds::wooden_profile
// 信息 ID: 426e3b0aa6d992b33036376c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=7844; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=23768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle
// 信息 ID: 145a97ff7cc6ef4fd5c9159d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=13260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$0
// 信息 ID: 2fb1d77c527eaa884600e764
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=15948; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=16192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$1
// 信息 ID: 6e105423233874862f88e290
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=16664; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=17428; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=18040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$2
// 信息 ID: 57f6b6eef9c7e53cbd7e37b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=18324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$3
// 信息 ID: 0d4870cfb2af716d6d9c67bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=19312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=19408; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=19496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$5
// 信息 ID: 36bb0ea616a2e40b3a0314dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_circle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5 <- system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>; module=3141, inline_record=1052; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>; module=3141, inline_record=1080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle
// 信息 ID: 110ab6b9ce9cad8b2fcca359
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$0
// 信息 ID: 5a693b6b180d178b9d62d3bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_rectangle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=1344; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=1496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$1
// 信息 ID: b055be627e408e5f2e869e6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_rectangle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::stable::driftsort_main<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1>,alloc::vec::Vec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::stable::drift::sort<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=14752; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=14888; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=15152; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=15308; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=15636; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=26128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=26708; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,alloc::slice::impl$0::sort_by::closure_env$0<system_wall_constructor::construct_platform_stairs::wooden::SideInterval,system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$1> >; module=1500, inline_record=80300; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$3
// 信息 ID: 8b3478e9eb40e7bcc0554a1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_rectangle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=6064; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=6432; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=6720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$5
// 信息 ID: fd697871215fd343f347e5a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::build_rectangle", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5 <- system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5 <- system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >; module=3141, inline_record=2684; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >; module=3141, inline_record=2740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs
// 信息 ID: b5d4b046791c5a9119cde4ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f1f21 -> 0x141a990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs::closure$0
// 信息 ID: c57937b2a99e954afcd7a47a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=26760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance
// 信息 ID: 76ab19d42aaa8f9aae1cceeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=1552; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=1880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance::closure$0
// 信息 ID: c113ec5ff0448c127d53771d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::wooden::perimeter_distance", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=1764; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=2092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::sample_profile_bounds
// 信息 ID: 03962a50b3b2f53dea91bc54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=6752; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=20856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::wooden::split_side_intervals
// 信息 ID: 559f44783e4a151b42761b6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs; module=1472, inline_record=2388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
