// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/stone.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 04e2f608729fc41fc009fb0ab4c9877cd5ea02ba44cc5c3db9f3b1c6dbb733e1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stone.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1459, 1473, 1482, 3144
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
// 函数符号: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1473,"record_offset":280,"rva":21094848,"va":5389803968}
// 编译实例 RVAs: 0x141e1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-14127795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e84d6d28ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389806478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=37
//     evidence={"assembly":"\tcallq\t-14126111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8e17328ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389804810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21084276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e88c47befe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389806463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=4
//     target_aliases=11 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20570771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e86d1dc6fe","size":5,"soft_fail":false,"target_va":5369240528,"va":5389811294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400beb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20321763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e81deac9fe","size":5,"soft_fail":false,"target_va":5369490256,"va":5389812014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16230918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8fa5508ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5389806545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11682392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8a8bd4dff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389809331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f2c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-182496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e82037fdff","size":5,"soft_fail":false,"target_va":5389626368,"va":5389808859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fa800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-148046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8b2bdfdff","size":5,"soft_fail":false,"target_va":5389658112,"va":5389806153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405920
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8f670feff","size":5,"soft_fail":false,"target_va":5389703456,"va":5389805605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-100505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e86777feff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389805172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414069a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-99436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8947bfeff","size":5,"soft_fail":false,"target_va":5389707680,"va":5389807111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e83bacffff","size":5,"soft_fail":false,"target_va":5389786032,"va":5389807472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14144c020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t182308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e824c80200","size":5,"soft_fail":false,"target_va":5389991968,"va":5389809655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26543193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e859049501","size":5,"soft_fail":false,"target_va":5416353040,"va":5389809842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27248484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e864c79f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5389808567,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27248776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e888c89f01","size":5,"soft_fail":false,"target_va":5417057088,"va":5389808307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27250206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e81ece9f01","size":5,"soft_fail":false,"target_va":5417057792,"va":5389807581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27253295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e82fda9f01","size":5,"soft_fail":false,"target_va":5417057840,"va":5389804540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bec0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27253599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e85fdb9f01","size":5,"soft_fail":false,"target_va":5417057984,"va":5389804380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26810006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e896169901","size":5,"soft_fail":false,"target_va":5416620769,"va":5389810758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26809707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e86b159901","size":5,"soft_fail":false,"target_va":5416620794,"va":5389811082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26810601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8e9189901","size":5,"soft_fail":false,"target_va":5416621600,"va":5389810994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26811556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8a41c9901","size":5,"soft_fail":false,"target_va":5416622208,"va":5389810647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26810657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e821199901","size":5,"soft_fail":false,"target_va":5416621762,"va":5389811100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26811678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e81e1d9901","size":5,"soft_fail":false,"target_va":5416622240,"va":5389810557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26810806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8b6199901","size":5,"soft_fail":false,"target_va":5416621632,"va":5389810821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26811838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8be1d9901","size":5,"soft_fail":false,"target_va":5416622320,"va":5389810477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8572632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e828317dff","size":5,"soft_fail":false,"target_va":5381234608,"va":5389807235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10585541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e83b7a5eff","size":5,"soft_fail":false,"target_va":5379218768,"va":5389804304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10588976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8d06c5eff","size":5,"soft_fail":false,"target_va":5379215376,"va":5389804347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10208194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e83e3c64ff","size":5,"soft_fail":false,"target_va":5379597392,"va":5389805581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_platform_stairs::add_colision_geo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12772826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8261a3dff","size":5,"soft_fail":false,"target_va":5377035056,"va":5389807877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6500415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8c1cf9cff","size":5,"soft_fail":false,"target_va":5383305360,"va":5389805770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6503951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8f1c19cff","size":5,"soft_fail":false,"target_va":5383304736,"va":5389808682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::circular_slice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6493974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8eae89cff","size":5,"soft_fail":false,"target_va":5383311600,"va":5389805569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6488612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8dcfd9cff","size":5,"soft_fail":false,"target_va":5383316912,"va":5389805519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6488292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e81cff9cff","size":5,"soft_fail":false,"target_va":5383320496,"va":5389808783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6530414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e8925a9cff","size":5,"soft_fail":false,"target_va":5383274080,"va":5389804489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6506380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389803968,"raw_hex":"e874b89cff","size":5,"soft_fail":false,"target_va":5383297984,"va":5389804359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1473,"record_offset":7336,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":724,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":10816,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":10612,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9755,"source_line":1761,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":12368,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":14644,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":12776,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1473,"record_offset":9096,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":1100,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5501,"source_line":1033,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$59::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":10500,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":37485,"source_line":2016,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1473,"record_offset":8380,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":10528,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":10416,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":9756,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1473,"record_offset":12060,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1473,"record_offset":6252,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1473,"record_offset":9672,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1473,"record_offset":1156,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1473,"record_offset":4308,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":10640,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":1128,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":1592,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":1344,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1473,"record_offset":948,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1473,"record_offset":836,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1473,"record_offset":14908,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11681,"source_line":201,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":6976,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":10240,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":10328,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":13232,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1473,"record_offset":1676,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1473,"record_offset":12172,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1473,"record_offset":10444,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1473,"record_offset":1044,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9805,"source_line":101,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::stone::stone_profile_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/stone.rs
//     evidence={"module":1473,"record_offset":12144,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79557,"source_line":266,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1473,"record_offset":808,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11668,"source_line":50,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":7520,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":3360,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":6948,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11669,"source_line":1078,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_new_from_points_remove_duplicates
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":13352,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":29027,"source_line":249,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1473,"record_offset":3064,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1473,"record_offset":1192,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":1220,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":2028,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":1564,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79540,"source_line":9,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":7396,"parent_record_offset":7364,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1473,"record_offset":4496,"parent_record_offset":4404,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1473,"record_offset":7364,"parent_record_offset":7336,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1473,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":280,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":11128,"parent_record_offset":11080,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":10944,"parent_record_offset":10916,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":12488,"parent_record_offset":12460,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":11004,"parent_record_offset":10972,"ancestor_record_offset":280,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":12548,"parent_record_offset":12516,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":12516,"parent_record_offset":12488,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1473,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1473,"record_offset":12396,"parent_record_offset":12368,"ancestor_record_offset":280,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1473,"record_offset":12424,"parent_record_offset":12396,"ancestor_record_offset":280,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":11712,"parent_record_offset":11488,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":10848,"parent_record_offset":10816,"ancestor_record_offset":280,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":10916,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":12460,"parent_record_offset":12424,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":3420,"parent_record_offset":3360,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1473,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1473,"record_offset":11420,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":1473,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":280,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1473,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":280,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1473,"record_offset":9820,"parent_record_offset":9784,"ancestor_record_offset":280,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1473,"record_offset":9852,"parent_record_offset":9820,"ancestor_record_offset":280,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1473,"record_offset":9984,"parent_record_offset":9884,"ancestor_record_offset":280,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1473,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":280,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1473,"record_offset":9168,"parent_record_offset":9144,"ancestor_record_offset":280,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":13124,"parent_record_offset":13100,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":11288,"parent_record_offset":11252,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1473,"record_offset":13100,"parent_record_offset":12936,"ancestor_record_offset":280,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":1256,"parent_record_offset":1220,"ancestor_record_offset":280,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":280,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::round
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":280,"inlinee":9806,"source_line":1782,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1473,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1473,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":280,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":280,"inlinee":9559,"source_line":154,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":9784,"parent_record_offset":9756,"ancestor_record_offset":280,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":280,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1473,"record_offset":13316,"parent_record_offset":13288,"ancestor_record_offset":280,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1473,"record_offset":11252,"parent_record_offset":11216,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1473,"record_offset":11216,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1473,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1473,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1473,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":280,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1473,"record_offset":4464,"parent_record_offset":4432,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1473,"record_offset":2196,"parent_record_offset":2144,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1473,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1473,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":280,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1473,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1473,"record_offset":9928,"parent_record_offset":9884,"ancestor_record_offset":280,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1473,"record_offset":9884,"parent_record_offset":9852,"ancestor_record_offset":280,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1473,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1473,"record_offset":3096,"parent_record_offset":3064,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":1473,"record_offset":9220,"parent_record_offset":9192,"ancestor_record_offset":280,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1473,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1473,"record_offset":12708,"parent_record_offset":12424,"ancestor_record_offset":280,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1473,"record_offset":13056,"parent_record_offset":13032,"ancestor_record_offset":280,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1473,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=61
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1473,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1473,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1473,"record_offset":13032,"parent_record_offset":13008,"ancestor_record_offset":280,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1473,"record_offset":1012,"parent_record_offset":948,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1473,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":280,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1473,"record_offset":984,"parent_record_offset":948,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1473,"record_offset":11332,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1473,"record_offset":7548,"parent_record_offset":7520,"ancestor_record_offset":280,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":280,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1473,"record_offset":12800,"parent_record_offset":12776,"ancestor_record_offset":280,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1473,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1473,"record_offset":10032,"parent_record_offset":10008,"ancestor_record_offset":280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1473,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1473,"record_offset":14728,"parent_record_offset":14700,"ancestor_record_offset":280,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1473,"record_offset":14700,"parent_record_offset":14672,"ancestor_record_offset":280,"inlinee":6155,"source_line":528,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1473,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1473,"record_offset":12824,"parent_record_offset":12800,"ancestor_record_offset":280,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1473,"record_offset":1708,"parent_record_offset":1676,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1473,"record_offset":10700,"parent_record_offset":10664,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1473,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":280,"inlinee":6686,"source_line":218,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":1473,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":280,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":7056,"parent_record_offset":7000,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9348,"parent_record_offset":9304,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9276,"parent_record_offset":9248,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9532,"parent_record_offset":9248,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1473,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":280,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1473,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":280,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1473,"record_offset":4180,"parent_record_offset":3996,"ancestor_record_offset":280,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1473,"record_offset":1796,"parent_record_offset":1676,"ancestor_record_offset":280,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1473,"record_offset":2084,"parent_record_offset":2028,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1473,"record_offset":7088,"parent_record_offset":7056,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/stone.rs
//     evidence={"module":1473,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":280,"inlinee":79437,"source_line":131,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1473,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1473,"record_offset":3516,"parent_record_offset":3360,"ancestor_record_offset":280,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1473,"record_offset":8436,"parent_record_offset":8408,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":2228,"parent_record_offset":2028,"ancestor_record_offset":280,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":1284,"parent_record_offset":1220,"ancestor_record_offset":280,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1473,"record_offset":8408,"parent_record_offset":8380,"ancestor_record_offset":280,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1473,"record_offset":3968,"parent_record_offset":3644,"ancestor_record_offset":280,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13714
// 首条位置: {"module":1482,"inlinee":79636}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1482,"record_offset":28828,"parent_record_offset":28292,"ancestor_record_offset":28292,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1482,"record_offset":28320,"parent_record_offset":28292,"ancestor_record_offset":28292,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1482,"record_offset":28348,"parent_record_offset":28292,"ancestor_record_offset":28292,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1482,"record_offset":28964,"parent_record_offset":28940,"ancestor_record_offset":28292,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1482,"record_offset":28988,"parent_record_offset":28964,"ancestor_record_offset":28292,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1482,"record_offset":28532,"parent_record_offset":28504,"ancestor_record_offset":28292,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":28472,"parent_record_offset":28392,"ancestor_record_offset":28292,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":28416,"parent_record_offset":28392,"ancestor_record_offset":28292,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1482,"record_offset":28940,"parent_record_offset":28916,"ancestor_record_offset":28292,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1482,"record_offset":28916,"parent_record_offset":28884,"ancestor_record_offset":28292,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1482,"record_offset":28392,"parent_record_offset":28348,"ancestor_record_offset":28292,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1482,"record_offset":28784,"parent_record_offset":28760,"ancestor_record_offset":28292,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1482,"record_offset":28884,"parent_record_offset":28856,"ancestor_record_offset":28292,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1482,"record_offset":28504,"parent_record_offset":28348,"ancestor_record_offset":28292,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1482,"record_offset":28760,"parent_record_offset":28736,"ancestor_record_offset":28292,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1482,"record_offset":28856,"parent_record_offset":28828,"ancestor_record_offset":28292,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1364D
// 首条位置: {"module":1459,"inlinee":79437}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9276,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9532,"parent_record_offset":9248,"ancestor_record_offset":9248,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1473,"record_offset":9452,"parent_record_offset":9428,"ancestor_record_offset":9248,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1473,"record_offset":9492,"parent_record_offset":9304,"ancestor_record_offset":9248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9388,"parent_record_offset":9348,"ancestor_record_offset":9248,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9348,"parent_record_offset":9304,"ancestor_record_offset":9248,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1473,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":9248,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1473,"record_offset":9428,"parent_record_offset":9388,"ancestor_record_offset":9248,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01473.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::stone::stone_profile_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136C5
// 首条位置: {"module":1473,"inlinee":79557}
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
// 完整结构及原始类型记录: stone.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stone.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/stone.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stone.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure$0
// 信息 ID: 242696123fd3756513f95d39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >,system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure_env$0<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > > >; module=1482, inline_record=28292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs::closure$1
// 信息 ID: de6f7db2776165191fa238b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=9248; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=9620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>
// 信息 ID: 61c88e9f606c02f551e6615f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs -> ["system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f2075 -> 0x141e1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::stone::stone_profile_t
// 信息 ID: 19e426f330f019cf69e40350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=12144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
