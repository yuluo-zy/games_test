// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/chimney_wall.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 435a97c1569e3d8c3eff6ab7c79a339bbf9bd0b2cf54655b36840bb7cd23f585
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: chimney_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 432, 1503, 1546
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
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":432,"record_offset":2044,"rva":8135088,"va":5376844208}
// 编译实例 RVAs: 0x7c21b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2169381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8dbe5deff","size":5,"soft_fail":false,"target_va":5374676688,"va":5376846064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1847531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e815cfe3ff","size":5,"soft_fail":false,"target_va":5375007712,"va":5376855238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1770238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e802fde4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376851721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1767552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e88007e5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376849051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t459549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e81d030700","size":5,"soft_fail":false,"target_va":5377309600,"va":5376850046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408392d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t485472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e860680700","size":5,"soft_fail":false,"target_va":5377331920,"va":5376846443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2464721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8d19b2500","size":5,"soft_fail":false,"target_va":5379310240,"va":5376845514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3548152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8f8233600","size":5,"soft_fail":false,"target_va":5380398224,"va":5376850067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3556635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e81b453600","size":5,"soft_fail":false,"target_va":5380401776,"va":5376845136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5039306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8cae44c00","size":5,"soft_fail":false,"target_va":5381889520,"va":5376850209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5042299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e87bf04c00","size":5,"soft_fail":false,"target_va":5381889824,"va":5376847520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11114198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8d696a900","size":5,"soft_fail":false,"target_va":5387964304,"va":5376850101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125fb20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11125033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e829c1a900","size":5,"soft_fail":false,"target_va":5387975456,"va":5376850418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11127293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8fdc9a900","size":5,"soft_fail":false,"target_va":5387976304,"va":5376849006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14126a450
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11171515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8bb76aa00","size":5,"soft_fail":false,"target_va":5388018768,"va":5376847248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412898e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11300150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8366dac00","size":5,"soft_fail":false,"target_va":5388146912,"va":5376846757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34994652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8dcf91502","size":5,"soft_fail":false,"target_va":5411847168,"va":5376852511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34687535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e82f4a1102","size":5,"soft_fail":false,"target_va":5411540992,"va":5376853452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t426598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e866820600","size":5,"soft_fail":false,"target_va":5377274800,"va":5376848197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34687306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e84a491102","size":5,"soft_fail":false,"target_va":5411540963,"va":5376853652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34688329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8494d1102","size":5,"soft_fail":false,"target_va":5411541584,"va":5376853250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34688753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8f14e1102","size":5,"soft_fail":false,"target_va":5411541776,"va":5376853018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34688699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8bb4e1102","size":5,"soft_fail":false,"target_va":5411542064,"va":5376853360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t34688615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8674e1102","size":5,"soft_fail":false,"target_va":5411542296,"va":5376853676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34688575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e83f4e1102","size":5,"soft_fail":false,"target_va":5411541904,"va":5376853324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e836fbffff","size":5,"soft_fail":false,"target_va":5376843136,"va":5376844357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34698921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8a9761102","size":5,"soft_fail":false,"target_va":5411543360,"va":5376844434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1483177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8a9a11600","size":5,"soft_fail":false,"target_va":5378331728,"va":5376848546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2379382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8764e2400","size":5,"soft_fail":false,"target_va":5379228288,"va":5376848901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2457642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e82a802500","size":5,"soft_fail":false,"target_va":5379303968,"va":5376846321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t460588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e82c070700","size":5,"soft_fail":false,"target_va":5377307232,"va":5376846639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2474049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e841c02500","size":5,"soft_fail":false,"target_va":5379319216,"va":5376845162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2463591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e867972500","size":5,"soft_fail":false,"target_va":5379310144,"va":5376846548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2459715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e843882500","size":5,"soft_fail":false,"target_va":5379305584,"va":5376845864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3555255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8b73f3600","size":5,"soft_fail":false,"target_va":5380401520,"va":5376846260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3570442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e80a7b3600","size":5,"soft_fail":false,"target_va":5380417056,"va":5376846609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3569856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8c0783600","size":5,"soft_fail":false,"target_va":5380417344,"va":5376847483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2946045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8fdf32c00","size":5,"soft_fail":false,"target_va":5379792624,"va":5376846574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1092816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8d0ac1000","size":5,"soft_fail":false,"target_va":5377945152,"va":5376852331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3237700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e844673100","size":5,"soft_fail":false,"target_va":5380088032,"va":5376850327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4540629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8d5484500","size":5,"soft_fail":false,"target_va":5381385536,"va":5376844902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4545818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e81a5d4500","size":5,"soft_fail":false,"target_va":5381390640,"va":5376844817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4541455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e80f4c4500","size":5,"soft_fail":false,"target_va":5381386480,"va":5376845020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4540362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8ca474500","size":5,"soft_fail":false,"target_va":5381384704,"va":5376844337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34791732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e834e11202","size":5,"soft_fail":false,"target_va":5411644800,"va":5376853063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11086574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8ee2aa900","size":5,"soft_fail":false,"target_va":5387942848,"va":5376856269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34683861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8d53b1102","size":5,"soft_fail":false,"target_va":5411537104,"va":5376853238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t397371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e83b100600","size":5,"soft_fail":false,"target_va":5377249968,"va":5376852592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t405138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376844208,"raw_hex":"e8922e0600","size":5,"soft_fail":false,"target_va":5377251504,"va":5376846361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":432,"record_offset":20868,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":432,"record_offset":21128,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":9540,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":15416,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":16428,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":432,"record_offset":9484,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":10448,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":432,"record_offset":6076,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":7544,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":6048,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":19816,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":9696,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":4740,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":432,"record_offset":16344,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":5992,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":432,"record_offset":10476,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":11748,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":3756,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":2320,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":2780,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_chimney
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":432,"record_offset":18820,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25122,"source_line":254,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":432,"record_offset":9568,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":432,"record_offset":12296,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25131,"source_line":277,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":432,"record_offset":15388,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":432,"record_offset":15360,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":432,"record_offset":16856,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25121,"source_line":152,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":432,"record_offset":15644,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25133,"source_line":24,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":9732,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25127,"source_line":1171,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":10044,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":11924,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":432,"record_offset":12268,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25129,"source_line":247,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":18948,"parent_record_offset":18924,"ancestor_record_offset":2044,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":2044,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":2044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":18892,"parent_record_offset":18856,"ancestor_record_offset":2044,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":18924,"parent_record_offset":18892,"ancestor_record_offset":2044,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":432,"record_offset":16644,"parent_record_offset":16556,"ancestor_record_offset":2044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":432,"record_offset":18856,"parent_record_offset":18820,"ancestor_record_offset":2044,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":432,"record_offset":19096,"parent_record_offset":19072,"ancestor_record_offset":2044,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":15488,"parent_record_offset":15456,"ancestor_record_offset":2044,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":20032,"parent_record_offset":20000,"ancestor_record_offset":2044,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":16584,"parent_record_offset":16556,"ancestor_record_offset":2044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":16556,"parent_record_offset":16532,"ancestor_record_offset":2044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":15536,"parent_record_offset":15512,"ancestor_record_offset":2044,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":15512,"parent_record_offset":15488,"ancestor_record_offset":2044,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":20100,"parent_record_offset":20064,"ancestor_record_offset":2044,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":20064,"parent_record_offset":20032,"ancestor_record_offset":2044,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":16532,"parent_record_offset":16508,"ancestor_record_offset":2044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":432,"record_offset":20140,"parent_record_offset":20100,"ancestor_record_offset":2044,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":15456,"parent_record_offset":15416,"ancestor_record_offset":2044,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":20408,"parent_record_offset":20384,"ancestor_record_offset":2044,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":19968,"parent_record_offset":19924,"ancestor_record_offset":2044,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":20000,"parent_record_offset":19968,"ancestor_record_offset":2044,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":10072,"parent_record_offset":10044,"ancestor_record_offset":2044,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":19852,"parent_record_offset":19816,"ancestor_record_offset":2044,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":432,"record_offset":20680,"parent_record_offset":20648,"ancestor_record_offset":2044,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":432,"record_offset":20792,"parent_record_offset":20680,"ancestor_record_offset":2044,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":432,"record_offset":20384,"parent_record_offset":19924,"ancestor_record_offset":2044,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":432,"record_offset":19888,"parent_record_offset":19852,"ancestor_record_offset":2044,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":432,"record_offset":19924,"parent_record_offset":19888,"ancestor_record_offset":2044,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":432,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":2044,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":432,"record_offset":20164,"parent_record_offset":20140,"ancestor_record_offset":2044,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":432,"record_offset":20188,"parent_record_offset":20164,"ancestor_record_offset":2044,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":2044,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":2044,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":2044,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2044,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4236,"parent_record_offset":4064,"ancestor_record_offset":2044,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":432,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":2044,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":432,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":2044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":432,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":2044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":432,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":2044,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":432,"record_offset":8704,"parent_record_offset":8680,"ancestor_record_offset":2044,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":432,"record_offset":6816,"parent_record_offset":6780,"ancestor_record_offset":2044,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":10920,"parent_record_offset":10500,"ancestor_record_offset":2044,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":10828,"parent_record_offset":10772,"ancestor_record_offset":2044,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":2044,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":2044,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":10628,"parent_record_offset":10544,"ancestor_record_offset":2044,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":432,"record_offset":10692,"parent_record_offset":10500,"ancestor_record_offset":2044,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":2044,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":9956,"parent_record_offset":9732,"ancestor_record_offset":2044,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":432,"record_offset":9664,"parent_record_offset":9568,"ancestor_record_offset":2044,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":432,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":2044,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":432,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":2044,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":432,"record_offset":9264,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":432,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":2044,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":432,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":2044,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":432,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":2044,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":2044,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":2044,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":432,"record_offset":20580,"parent_record_offset":20480,"ancestor_record_offset":2044,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":20544,"parent_record_offset":20516,"ancestor_record_offset":2044,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":432,"record_offset":20516,"parent_record_offset":20480,"ancestor_record_offset":2044,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":432,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":2044,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":13272,"parent_record_offset":12296,"ancestor_record_offset":2044,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":14008,"parent_record_offset":13308,"ancestor_record_offset":2044,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":20480,"parent_record_offset":20456,"ancestor_record_offset":2044,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":2044,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":20648,"parent_record_offset":20580,"ancestor_record_offset":2044,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":13308,"parent_record_offset":13272,"ancestor_record_offset":2044,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":432,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2044,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":2044,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":20236,"parent_record_offset":20212,"ancestor_record_offset":2044,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":432,"record_offset":16612,"parent_record_offset":16584,"ancestor_record_offset":2044,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":432,"record_offset":2480,"parent_record_offset":2448,"ancestor_record_offset":2044,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":432,"record_offset":6616,"parent_record_offset":6584,"ancestor_record_offset":2044,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":432,"record_offset":6584,"parent_record_offset":6552,"ancestor_record_offset":2044,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":12208,"parent_record_offset":11924,"ancestor_record_offset":2044,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":10376,"parent_record_offset":10348,"ancestor_record_offset":2044,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":11776,"parent_record_offset":11748,"ancestor_record_offset":2044,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":12152,"parent_record_offset":11924,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":432,"record_offset":3908,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":432,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":2044,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":6140,"parent_record_offset":6108,"ancestor_record_offset":2044,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":7212,"parent_record_offset":6220,"ancestor_record_offset":2044,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":7236,"parent_record_offset":7212,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":7092,"parent_record_offset":6300,"ancestor_record_offset":2044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":7272,"parent_record_offset":6196,"ancestor_record_offset":2044,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":9440,"parent_record_offset":8372,"ancestor_record_offset":2044,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":14196,"parent_record_offset":14168,"ancestor_record_offset":2044,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":4696,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":4300,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":16392,"parent_record_offset":16368,"ancestor_record_offset":2044,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":2044,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2044,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2044,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":432,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":2044,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":19000,"parent_record_offset":18972,"ancestor_record_offset":2044,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":432,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":2044,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":2044,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":10128,"parent_record_offset":10100,"ancestor_record_offset":2044,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":432,"record_offset":16368,"parent_record_offset":16344,"ancestor_record_offset":2044,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2044,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2044,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":432,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":432,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2044,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":432,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":2044,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":432,"record_offset":2948,"parent_record_offset":2916,"ancestor_record_offset":2044,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":432,"record_offset":2916,"parent_record_offset":2880,"ancestor_record_offset":2044,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":432,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2044,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":432,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2044,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":432,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2044,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":432,"record_offset":3420,"parent_record_offset":2880,"ancestor_record_offset":2044,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2420,"parent_record_offset":2384,"ancestor_record_offset":2044,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":2044,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2044,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2676,"parent_record_offset":2580,"ancestor_record_offset":2044,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2044,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2356,"parent_record_offset":2320,"ancestor_record_offset":2044,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":432,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":2044,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":432,"record_offset":2980,"parent_record_offset":2948,"ancestor_record_offset":2044,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":432,"record_offset":3264,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":432,"record_offset":3236,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":432,"record_offset":3040,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":432,"record_offset":21156,"parent_record_offset":21128,"ancestor_record_offset":2044,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":432,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":2044,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":432,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":2044,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":432,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":2044,"inlinee":24786,"source_line":746,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":432,"record_offset":8372,"parent_record_offset":8336,"ancestor_record_offset":2044,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14288,"parent_record_offset":14168,"ancestor_record_offset":2044,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14168,"parent_record_offset":14132,"ancestor_record_offset":2044,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14132,"parent_record_offset":14044,"ancestor_record_offset":2044,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14228,"parent_record_offset":14196,"ancestor_record_offset":2044,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":432,"record_offset":16308,"parent_record_offset":15668,"ancestor_record_offset":2044,"inlinee":25144,"source_line":106,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":432,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":2044,"inlinee":25134,"source_line":218,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":432,"record_offset":15712,"parent_record_offset":15668,"ancestor_record_offset":2044,"inlinee":25135,"source_line":184,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":432,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":2044,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":432,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":2044,"inlinee":25137,"source_line":1011,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":432,"record_offset":10948,"parent_record_offset":10500,"ancestor_record_offset":2044,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":432,"record_offset":15896,"parent_record_offset":15712,"ancestor_record_offset":2044,"inlinee":18181,"source_line":91,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$100::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":432,"record_offset":15856,"parent_record_offset":15832,"ancestor_record_offset":2044,"inlinee":25139,"source_line":1784,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$103::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":432,"record_offset":16252,"parent_record_offset":15668,"ancestor_record_offset":2044,"inlinee":25143,"source_line":1811,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":432,"record_offset":16000,"parent_record_offset":15668,"ancestor_record_offset":2044,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":10320,"parent_record_offset":10292,"ancestor_record_offset":2044,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":432,"record_offset":10348,"parent_record_offset":10320,"ancestor_record_offset":2044,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":432,"record_offset":10572,"parent_record_offset":10544,"ancestor_record_offset":2044,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":432,"record_offset":10544,"parent_record_offset":10500,"ancestor_record_offset":2044,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":6512,"parent_record_offset":6472,"ancestor_record_offset":2044,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":6552,"parent_record_offset":6512,"ancestor_record_offset":2044,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":6660,"parent_record_offset":6472,"ancestor_record_offset":2044,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":6472,"parent_record_offset":6420,"ancestor_record_offset":2044,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":8680,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":6696,"parent_record_offset":6420,"ancestor_record_offset":2044,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":9292,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":6780,"parent_record_offset":6752,"ancestor_record_offset":2044,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":6752,"parent_record_offset":6420,"ancestor_record_offset":2044,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":8764,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":432,"record_offset":8652,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":432,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":2044,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":8432,"parent_record_offset":8404,"ancestor_record_offset":2044,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":2044,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":2044,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":9148,"parent_record_offset":8980,"ancestor_record_offset":2044,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":6352,"parent_record_offset":6324,"ancestor_record_offset":2044,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":2044,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":8492,"parent_record_offset":8460,"ancestor_record_offset":2044,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":2044,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":7000,"parent_record_offset":6352,"ancestor_record_offset":2044,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":9032,"parent_record_offset":9004,"ancestor_record_offset":2044,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":6852,"parent_record_offset":6420,"ancestor_record_offset":2044,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":9384,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13072,"parent_record_offset":13012,"ancestor_record_offset":2044,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":6420,"parent_record_offset":6384,"ancestor_record_offset":2044,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":9004,"parent_record_offset":8980,"ancestor_record_offset":2044,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":2044,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":8520,"parent_record_offset":8460,"ancestor_record_offset":2044,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":2044,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13044,"parent_record_offset":13012,"ancestor_record_offset":2044,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":2044,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13012,"parent_record_offset":12988,"ancestor_record_offset":2044,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":8624,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":6384,"parent_record_offset":6352,"ancestor_record_offset":2044,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":8980,"parent_record_offset":8568,"ancestor_record_offset":2044,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":432,"record_offset":7600,"parent_record_offset":7576,"ancestor_record_offset":2044,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":3792,"parent_record_offset":3756,"ancestor_record_offset":2044,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":4804,"parent_record_offset":4772,"ancestor_record_offset":2044,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":5320,"parent_record_offset":5060,"ancestor_record_offset":2044,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":432,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":2044,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":4012,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":432,"record_offset":4272,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":432,"record_offset":8404,"parent_record_offset":8372,"ancestor_record_offset":2044,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":432,"record_offset":12940,"parent_record_offset":12296,"ancestor_record_offset":2044,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":432,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2044,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":432,"record_offset":7576,"parent_record_offset":7544,"ancestor_record_offset":2044,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":432,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":2044,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":432,"record_offset":3864,"parent_record_offset":3828,"ancestor_record_offset":2044,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":432,"record_offset":3828,"parent_record_offset":3792,"ancestor_record_offset":2044,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":432,"record_offset":20608,"parent_record_offset":20580,"ancestor_record_offset":2044,"inlinee":25158,"source_line":259,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":432,"record_offset":14044,"parent_record_offset":14008,"ancestor_record_offset":2044,"inlinee":25146,"source_line":302,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14416,"parent_record_offset":14380,"ancestor_record_offset":2044,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":2044,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":12376,"parent_record_offset":12296,"ancestor_record_offset":2044,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":11968,"parent_record_offset":11924,"ancestor_record_offset":2044,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":10168,"parent_record_offset":10044,"ancestor_record_offset":2044,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":10012,"parent_record_offset":9732,"ancestor_record_offset":2044,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9984,"parent_record_offset":9732,"ancestor_record_offset":2044,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9780,"parent_record_offset":9732,"ancestor_record_offset":2044,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9816,"parent_record_offset":9780,"ancestor_record_offset":2044,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":2044,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":9892,"parent_record_offset":9732,"ancestor_record_offset":2044,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":12648,"parent_record_offset":12296,"ancestor_record_offset":2044,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14340,"parent_record_offset":14288,"ancestor_record_offset":2044,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":2044,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":432,"record_offset":10292,"parent_record_offset":10044,"ancestor_record_offset":2044,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":432,"record_offset":14380,"parent_record_offset":14044,"ancestor_record_offset":2044,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11882
// 首条位置: {"module":1546,"inlinee":71810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x116F3
// 首条位置: {"module":1503,"inlinee":71411}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11886
// 首条位置: {"module":1546,"inlinee":71814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat4.rs
//     evidence={"module":1546,"record_offset":29144,"parent_record_offset":29104,"ancestor_record_offset":29104,"inlinee":18170,"source_line":1227,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1546,"record_offset":30476,"parent_record_offset":29104,"ancestor_record_offset":29104,"inlinee":30322,"source_line":199,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":29368,"parent_record_offset":29272,"ancestor_record_offset":29104,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":29240,"parent_record_offset":29176,"ancestor_record_offset":29104,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":29208,"parent_record_offset":29176,"ancestor_record_offset":29104,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1546,"record_offset":29528,"parent_record_offset":29144,"ancestor_record_offset":29104,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1546,"record_offset":29176,"parent_record_offset":29144,"ancestor_record_offset":29104,"inlinee":18172,"source_line":547,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1546,"record_offset":29272,"parent_record_offset":29144,"ancestor_record_offset":29104,"inlinee":18173,"source_line":1051,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1546,"record_offset":29400,"parent_record_offset":29144,"ancestor_record_offset":29104,"inlinee":18174,"source_line":1561,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6246
// 首条位置: {"module":432,"inlinee":25158}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x622B
// 首条位置: {"module":432,"inlinee":25131}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":13272,"parent_record_offset":12296,"ancestor_record_offset":12296,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":432,"record_offset":12940,"parent_record_offset":12296,"ancestor_record_offset":12296,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":12376,"parent_record_offset":12296,"ancestor_record_offset":12296,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":12648,"parent_record_offset":12296,"ancestor_record_offset":12296,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":432,"record_offset":13128,"parent_record_offset":13100,"ancestor_record_offset":12296,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":432,"record_offset":13188,"parent_record_offset":13156,"ancestor_record_offset":12296,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":12296,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":432,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":12296,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":14008,"parent_record_offset":13308,"ancestor_record_offset":12296,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":432,"record_offset":13308,"parent_record_offset":13272,"ancestor_record_offset":12296,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":432,"record_offset":13616,"parent_record_offset":13584,"ancestor_record_offset":12296,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":432,"record_offset":13584,"parent_record_offset":13552,"ancestor_record_offset":12296,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":12908,"parent_record_offset":12648,"ancestor_record_offset":12296,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":12880,"parent_record_offset":12648,"ancestor_record_offset":12296,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":432,"record_offset":12852,"parent_record_offset":12648,"ancestor_record_offset":12296,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":12496,"parent_record_offset":12456,"ancestor_record_offset":12296,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":14196,"parent_record_offset":14168,"ancestor_record_offset":12296,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":432,"record_offset":13224,"parent_record_offset":13072,"ancestor_record_offset":12296,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":432,"record_offset":13876,"parent_record_offset":13852,"ancestor_record_offset":12296,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":12696,"parent_record_offset":12648,"ancestor_record_offset":12296,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":432,"record_offset":12724,"parent_record_offset":12696,"ancestor_record_offset":12296,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":432,"record_offset":14100,"parent_record_offset":14044,"ancestor_record_offset":12296,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14288,"parent_record_offset":14168,"ancestor_record_offset":12296,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14168,"parent_record_offset":14132,"ancestor_record_offset":12296,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14132,"parent_record_offset":14044,"ancestor_record_offset":12296,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14228,"parent_record_offset":14196,"ancestor_record_offset":12296,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":13512,"parent_record_offset":13472,"ancestor_record_offset":12296,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":13552,"parent_record_offset":13512,"ancestor_record_offset":12296,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":13660,"parent_record_offset":13472,"ancestor_record_offset":12296,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":432,"record_offset":13472,"parent_record_offset":13424,"ancestor_record_offset":12296,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":13100,"parent_record_offset":13072,"ancestor_record_offset":12296,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":13156,"parent_record_offset":13100,"ancestor_record_offset":12296,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":432,"record_offset":13752,"parent_record_offset":13424,"ancestor_record_offset":12296,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":12296,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":432,"record_offset":13372,"parent_record_offset":13348,"ancestor_record_offset":12296,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13852,"parent_record_offset":13424,"ancestor_record_offset":12296,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13072,"parent_record_offset":13012,"ancestor_record_offset":12296,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13424,"parent_record_offset":13396,"ancestor_record_offset":12296,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":12296,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13044,"parent_record_offset":13012,"ancestor_record_offset":12296,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13012,"parent_record_offset":12988,"ancestor_record_offset":12296,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":432,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":12296,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":432,"record_offset":13348,"parent_record_offset":13308,"ancestor_record_offset":12296,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":432,"record_offset":14044,"parent_record_offset":14008,"ancestor_record_offset":12296,"inlinee":25146,"source_line":302,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14416,"parent_record_offset":14380,"ancestor_record_offset":12296,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":12296,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":432,"record_offset":12760,"parent_record_offset":12648,"ancestor_record_offset":12296,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12616,"parent_record_offset":12376,"ancestor_record_offset":12296,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12588,"parent_record_offset":12376,"ancestor_record_offset":12296,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12420,"parent_record_offset":12376,"ancestor_record_offset":12296,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12456,"parent_record_offset":12420,"ancestor_record_offset":12296,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":12296,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":432,"record_offset":12532,"parent_record_offset":12376,"ancestor_record_offset":12296,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14340,"parent_record_offset":14288,"ancestor_record_offset":12296,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":12296,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":432,"record_offset":14380,"parent_record_offset":14044,"ancestor_record_offset":12296,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x623A
// 首条位置: {"module":432,"inlinee":25146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":432,"record_offset":14100,"parent_record_offset":14044,"ancestor_record_offset":14044,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14132,"parent_record_offset":14044,"ancestor_record_offset":14044,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":432,"record_offset":14380,"parent_record_offset":14044,"ancestor_record_offset":14044,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":432,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":14044,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":432,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":14044,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":432,"record_offset":14196,"parent_record_offset":14168,"ancestor_record_offset":14044,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14288,"parent_record_offset":14168,"ancestor_record_offset":14044,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14168,"parent_record_offset":14132,"ancestor_record_offset":14044,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":432,"record_offset":14228,"parent_record_offset":14196,"ancestor_record_offset":14044,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14416,"parent_record_offset":14380,"ancestor_record_offset":14044,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":432,"record_offset":14440,"parent_record_offset":14416,"ancestor_record_offset":14044,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14340,"parent_record_offset":14288,"ancestor_record_offset":14044,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":432,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":14044,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00432.jsonl.gz"}
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
// 完整结构及原始类型记录: chimney_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: chimney_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: chimney_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys
// 信息 ID: e3b7bddf014d0b698e0fd58e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x15190b -> 0x7c21b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x439fcf -> 0x7c21b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x47f178 -> 0x7c21b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$0
// 信息 ID: 0a56308e3f72af043c1da4d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$1
// 信息 ID: 613bbfb40019ef029cead786
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$1>,alloc::alloc::Global>; module=1503, inline_record=13952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$3
// 信息 ID: 4f62f614c11d35a1c1b2085e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$31::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$33::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$20::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; module=1546, inline_record=29104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
// 信息 ID: db9f424ea8e3aec8ac0aaf2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=20608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement
// 信息 ID: b519f42fe4932ab565cc22bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=12296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
// 信息 ID: 7d65e46be7a35dc230a28a9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::is_valid_body_placement", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=432, inline_record=14044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
