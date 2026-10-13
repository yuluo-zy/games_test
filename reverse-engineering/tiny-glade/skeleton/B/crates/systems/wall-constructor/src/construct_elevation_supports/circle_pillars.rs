// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4c26a53b47b7b7dc1dfd578065b05963d6517d28042da28df1709f89bb4f8ed8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_pillars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1479
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
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
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1479,"record_offset":272,"rva":19184176,"va":5387893296}
// 编译实例 RVAs: 0x124ba30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18627610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8e6c3e3fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387895269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12814351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8f1773cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387895850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9989459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8ad9267ff","size":5,"soft_fail":false,"target_va":5377906288,"va":5387895742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7836610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e83e6c88ff","size":5,"soft_fail":false,"target_va":5380057424,"va":5387894029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6010489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e88749a4ff","size":5,"soft_fail":false,"target_va":5381883040,"va":5387893524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5959620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e83c10a5ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387894143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5959526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e89a10a5ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387894081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e809b3ffff","size":5,"soft_fail":false,"target_va":5387874224,"va":5387893922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23953123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8e37e6d01","size":5,"soft_fail":false,"target_va":5411847952,"va":5387894824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_pillars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23954502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e846846d01","size":5,"soft_fail":false,"target_va":5411848096,"va":5387893589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13539704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8886631ff","size":5,"soft_fail":false,"target_va":5374355376,"va":5387895075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23645581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e88dcd6801","size":5,"soft_fail":false,"target_va":5411541776,"va":5387896190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23645893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8c5ce6801","size":5,"soft_fail":false,"target_va":5411542064,"va":5387896166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23645769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e849ce6801","size":5,"soft_fail":false,"target_va":5411541904,"va":5387896130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7477027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8dde88dff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387893998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8101355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8156284ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5387893974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::rectangle_pillars::find_stone_column_start_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e82a490000","size":5,"soft_fail":false,"target_va":5387912992,"va":5387894257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14249404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8bc6dd900","size":5,"soft_fail":false,"target_va":5402145344,"va":5387895935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14245187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8435dd900","size":5,"soft_fail":false,"target_va":5402141008,"va":5387895816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5981309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e883bba4ff","size":5,"soft_fail":false,"target_va":5381912800,"va":5387894104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5981903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e831b9a4ff","size":5,"soft_fail":false,"target_va":5381911968,"va":5387893866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5980554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e876bea4ff","size":5,"soft_fail":false,"target_va":5381912944,"va":5387893493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::signed_distance
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5979719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8b9c1a4ff","size":5,"soft_fail":false,"target_va":5381914912,"va":5387894626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5969890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e81ee8a4ff","size":5,"soft_fail":false,"target_va":5381924688,"va":5387894573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5980131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e81dc0a4ff","size":5,"soft_fail":false,"target_va":5381915520,"va":5387895646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5988369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387893296,"raw_hex":"e8ef9fa4ff","size":5,"soft_fail":false,"target_va":5381907008,"va":5387895372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1479,"record_offset":536,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1479,"record_offset":3136,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1479,"record_offset":2444,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1479,"record_offset":564,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1479,"record_offset":1744,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1479,"record_offset":764,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1479,"record_offset":1120,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1479,"record_offset":2016,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1479,"record_offset":3780,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1479,"record_offset":3328,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1479,"record_offset":1060,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1828,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1480,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1592,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1479,"record_offset":3752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1479,"record_offset":1156,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1479,"record_offset":1244,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1479,"record_offset":596,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1479,"record_offset":736,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6386,"source_line":98,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
//     evidence={"module":1479,"record_offset":1332,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":71262,"source_line":54,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1479,"record_offset":848,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1479,"record_offset":2320,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1479,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1479,"record_offset":3944,"parent_record_offset":3852,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3208,"parent_record_offset":3176,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3256,"parent_record_offset":3232,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1479,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1479,"record_offset":3176,"parent_record_offset":3136,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1479,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":272,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1479,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":272,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1479,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":272,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1479,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":272,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1479,"record_offset":1000,"parent_record_offset":848,"ancestor_record_offset":272,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1479,"record_offset":1028,"parent_record_offset":848,"ancestor_record_offset":272,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1479,"record_offset":788,"parent_record_offset":764,"ancestor_record_offset":272,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1479,"record_offset":3912,"parent_record_offset":3880,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":704,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":648,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":620,"parent_record_offset":596,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1479,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":272,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1479,"record_offset":888,"parent_record_offset":848,"ancestor_record_offset":272,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1479,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1479,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1479,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":272,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1479,"record_offset":3296,"parent_record_offset":3136,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1479,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1912,"parent_record_offset":1880,"ancestor_record_offset":272,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":272,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":272,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2124,"parent_record_offset":2044,"ancestor_record_offset":272,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":272,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1479,"record_offset":1420,"parent_record_offset":1332,"ancestor_record_offset":272,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1479,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_pillars.rs
//     evidence={"module":1479,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":272,"inlinee":71263,"source_line":135,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1479,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":272,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1479,"record_offset":2508,"parent_record_offset":2476,"ancestor_record_offset":272,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1479,"record_offset":2384,"parent_record_offset":2320,"ancestor_record_offset":272,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1479,"record_offset":2476,"parent_record_offset":2444,"ancestor_record_offset":272,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1479,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":272,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1165E
// 首条位置: {"module":1479,"inlinee":71262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1479,"record_offset":1420,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1479,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1479,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":1332,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1332,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1165F
// 首条位置: {"module":1479,"inlinee":71263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2124,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":2276,"parent_record_offset":2148,"ancestor_record_offset":2044,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1479,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2208,"parent_record_offset":2176,"ancestor_record_offset":2044,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2176,"parent_record_offset":2148,"ancestor_record_offset":2044,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1479,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":2044,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01479.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1479,"record_offset":2240,"parent_record_offset":2208,"ancestor_record_offset":2044,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01479.jsonl.gz"}
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
// 完整结构及原始类型记录: circle_pillars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_pillars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_pillars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars
// 信息 ID: 02187f59b502d25c08b7ad11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x788601 -> 0x124ba30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$0
// 信息 ID: 6d578a008e8c679371ba9af5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1479, inline_record=1332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars::closure$1
// 信息 ID: c3481d49502e6610d9d19a3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars; module=1479, inline_record=2044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
