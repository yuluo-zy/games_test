// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/chimney_wall.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1c6462a52e5e7196d4b6d6418c0662695858f8ba439ffb6a82af7abadcf7561f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: chimney_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2756, 2761, 3865
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3865,"record_offset":3060,"rva":29776672,"va":5398485792}
// 编译实例 RVAs: 0x1c65b20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26070859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8b53072fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398496166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25743299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e83d3077fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398492990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25740825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8e73977fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398490532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23916645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e89b0f93fe","size":5,"soft_fail":false,"target_va":5374572480,"va":5398489120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23770262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e86a4b95fe","size":5,"soft_fail":false,"target_va":5374716816,"va":5398487073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23279373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8f3c89cfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398489144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23273506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8dedf9cfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398486733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16233532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8c44b08ff","size":5,"soft_fail":false,"target_va":5382255760,"va":5398489287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16232437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e80b5008ff","size":5,"soft_fail":false,"target_va":5382256208,"va":5398488640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16199624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e838d008ff","size":5,"soft_fail":false,"target_va":5382288704,"va":5398488323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16162760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8386009ff","size":5,"soft_fail":false,"target_va":5382330128,"va":5398492883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16122175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8c1fe09ff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398489178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06920
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16120330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8f6050aff","size":5,"soft_fail":false,"target_va":5382367520,"va":5398487845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417af100
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4947962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e80680b4ff","size":5,"soft_fail":false,"target_va":5393543424,"va":5398491381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417af4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4946124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e83487b4ff","size":5,"soft_fail":false,"target_va":5393544368,"va":5398490487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4483929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8596b4400","size":5,"soft_fail":false,"target_va":5402978144,"va":5398494210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5111305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8f701b2ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398489700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20967293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e88310c0fe","size":5,"soft_fail":false,"target_va":5377527120,"va":5398494408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4484108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e80c6c4400","size":5,"soft_fail":false,"target_va":5402978688,"va":5398494575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4484443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e85b6d4400","size":5,"soft_fail":false,"target_va":5402978880,"va":5398494432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4484901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8256f4400","size":5,"soft_fail":false,"target_va":5402979152,"va":5398494246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t4485417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e829714400","size":5,"soft_fail":false,"target_va":5402979344,"va":5398493922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4484749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e88d6e4400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398494190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8b8f7ffff","size":5,"soft_fail":false,"target_va":5398484512,"va":5398486627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4493741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8ad914400","size":5,"soft_fail":false,"target_va":5402980128,"va":5398486382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e827fbffff","size":5,"soft_fail":false,"target_va":5398484704,"va":5398485940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4494634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e82a954400","size":5,"soft_fail":false,"target_va":5402980656,"va":5398486017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23922102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e84afa92fe","size":5,"soft_fail":false,"target_va":5374567936,"va":5398490033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::as_triangles
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13919546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8c69a2bff","size":5,"soft_fail":false,"target_va":5384570848,"va":5398490389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24124848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e850e28ffe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398487419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23916740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e83c0f93fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398487727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23768063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8015495fe","size":5,"soft_fail":false,"target_va":5374718704,"va":5398486762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23770944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8c04895fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5398487643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23274672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e850db9cfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398487355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16554845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8a36403ff","size":5,"soft_fail":false,"target_va":5381932864,"va":5398487704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16555523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8fd6103ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398488574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23052698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8663ea0fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5398487669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_indexed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23976570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e8862592fe","size":5,"soft_fail":false,"target_va":5374516688,"va":5398493253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24205222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e85aa88efe","size":5,"soft_fail":false,"target_va":5374286080,"va":5398491297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22129647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e81154aefe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398486554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22125755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e84563aefe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398486310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22129781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e88b53aefe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398485920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16196527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e851dc08ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398486330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5002898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e86ea9b3ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398493565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4995983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398485792,"raw_hex":"e871c4b3ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398487466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3865,"record_offset":18592,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3865,"record_offset":18852,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":8100,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":13772,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":14728,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3865,"record_offset":8044,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3865,"record_offset":8828,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3865,"record_offset":6080,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3865,"record_offset":6052,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":17628,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":8248,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3865,"record_offset":14784,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3865,"record_offset":14644,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":5996,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3865,"record_offset":8856,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3865,"record_offset":10688,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3865,"record_offset":10632,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3865,"record_offset":10112,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":4948,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":3348,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":3936,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":17600,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":7088,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":7448,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3865,"record_offset":8128,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":3865,"record_offset":10772,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":111892,"source_line":287,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3865,"record_offset":13744,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3865,"record_offset":13716,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3865,"record_offset":15132,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":89814,"source_line":135,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_matrix
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3865,"record_offset":13944,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":27549,"source_line":23,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":8284,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":59684,"source_line":954,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":8576,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":10288,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3865,"record_offset":10744,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":60026,"source_line":231,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":17972,"parent_record_offset":17948,"ancestor_record_offset":3060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":17996,"parent_record_offset":17972,"ancestor_record_offset":3060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":18664,"parent_record_offset":18628,"ancestor_record_offset":3060,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":17948,"parent_record_offset":17848,"ancestor_record_offset":3060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3865,"record_offset":14924,"parent_record_offset":14832,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3865,"record_offset":18628,"parent_record_offset":18592,"ancestor_record_offset":3060,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":14860,"parent_record_offset":14832,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":13844,"parent_record_offset":13812,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":3060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":17820,"parent_record_offset":17792,"ancestor_record_offset":3060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":18068,"parent_record_offset":17820,"ancestor_record_offset":3060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3865,"record_offset":14832,"parent_record_offset":14808,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":13812,"parent_record_offset":13772,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":3060,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":17764,"parent_record_offset":17724,"ancestor_record_offset":3060,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":17792,"parent_record_offset":17764,"ancestor_record_offset":3060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":8604,"parent_record_offset":8576,"ancestor_record_offset":3060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":17660,"parent_record_offset":17628,"ancestor_record_offset":3060,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3865,"record_offset":18408,"parent_record_offset":18376,"ancestor_record_offset":3060,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3865,"record_offset":18516,"parent_record_offset":18408,"ancestor_record_offset":3060,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3865,"record_offset":18112,"parent_record_offset":17724,"ancestor_record_offset":3060,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":3865,"record_offset":17692,"parent_record_offset":17660,"ancestor_record_offset":3060,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":3865,"record_offset":17724,"parent_record_offset":17692,"ancestor_record_offset":3060,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3865,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3865,"record_offset":17884,"parent_record_offset":17848,"ancestor_record_offset":3060,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3865,"record_offset":17912,"parent_record_offset":17884,"ancestor_record_offset":3060,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5492,"parent_record_offset":5320,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3865,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3865,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":3060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3865,"record_offset":6508,"parent_record_offset":6476,"ancestor_record_offset":3060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3865,"record_offset":6572,"parent_record_offset":6536,"ancestor_record_offset":3060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":9296,"parent_record_offset":8880,"ancestor_record_offset":3060,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":9204,"parent_record_offset":9148,"ancestor_record_offset":3060,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":3060,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":14324,"parent_record_offset":14300,"ancestor_record_offset":3060,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":9008,"parent_record_offset":8924,"ancestor_record_offset":3060,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3865,"record_offset":9068,"parent_record_offset":8880,"ancestor_record_offset":3060,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3865,"record_offset":8488,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3865,"record_offset":8216,"parent_record_offset":8128,"ancestor_record_offset":3060,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3865,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":3060,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3865,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3060,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3865,"record_offset":7024,"parent_record_offset":7000,"ancestor_record_offset":3060,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3865,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":3060,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3865,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":3060,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3865,"record_offset":18308,"parent_record_offset":18208,"ancestor_record_offset":3060,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3865,"record_offset":18272,"parent_record_offset":18244,"ancestor_record_offset":3060,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3865,"record_offset":18244,"parent_record_offset":18208,"ancestor_record_offset":3060,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3865,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":3060,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":11724,"parent_record_offset":10772,"ancestor_record_offset":3060,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":12388,"parent_record_offset":11756,"ancestor_record_offset":3060,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":18208,"parent_record_offset":18184,"ancestor_record_offset":3060,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":3060,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":18376,"parent_record_offset":18308,"ancestor_record_offset":3060,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":11756,"parent_record_offset":11724,"ancestor_record_offset":3060,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3865,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3865,"record_offset":14888,"parent_record_offset":14860,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3865,"record_offset":3572,"parent_record_offset":3476,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3865,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":3060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":10544,"parent_record_offset":10288,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":10572,"parent_record_offset":10288,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":3060,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":10140,"parent_record_offset":10112,"ancestor_record_offset":3060,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":10516,"parent_record_offset":10288,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3865,"record_offset":6168,"parent_record_offset":6104,"ancestor_record_offset":3060,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3865,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":3060,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":6140,"parent_record_offset":6104,"ancestor_record_offset":3060,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":5184,"parent_record_offset":5156,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":6940,"parent_record_offset":6104,"ancestor_record_offset":3060,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":4532,"parent_record_offset":4072,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":5232,"parent_record_offset":5056,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":7000,"parent_record_offset":6080,"ancestor_record_offset":3060,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":8388,"parent_record_offset":8356,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":12536,"parent_record_offset":12508,"ancestor_record_offset":3060,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3865,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3865,"record_offset":5556,"parent_record_offset":5260,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3865,"record_offset":14692,"parent_record_offset":14668,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3865,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":3060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3865,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":3060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3865,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3865,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3865,"record_offset":4492,"parent_record_offset":4460,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3865,"record_offset":18024,"parent_record_offset":17996,"ancestor_record_offset":3060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3865,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3865,"record_offset":5956,"parent_record_offset":5020,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3865,"record_offset":3508,"parent_record_offset":3476,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3865,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":3060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3865,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3865,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":3060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3865,"record_offset":14668,"parent_record_offset":14644,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3865,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3865,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3865,"record_offset":4236,"parent_record_offset":4128,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3865,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3865,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3865,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3865,"record_offset":4072,"parent_record_offset":4040,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3865,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3865,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3865,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3865,"record_offset":4564,"parent_record_offset":4040,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3832,"parent_record_offset":3672,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3384,"parent_record_offset":3348,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3448,"parent_record_offset":3412,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3865,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3865,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3865,"record_offset":4376,"parent_record_offset":4128,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3865,"record_offset":4348,"parent_record_offset":4128,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3865,"record_offset":4180,"parent_record_offset":4128,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3865,"record_offset":18880,"parent_record_offset":18852,"ancestor_record_offset":3060,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3865,"record_offset":18904,"parent_record_offset":18880,"ancestor_record_offset":3060,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3865,"record_offset":12876,"parent_record_offset":12420,"ancestor_record_offset":3060,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3865,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":3060,"inlinee":25252,"source_line":180,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12628,"parent_record_offset":12508,"ancestor_record_offset":3060,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12508,"parent_record_offset":12472,"ancestor_record_offset":3060,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12472,"parent_record_offset":12420,"ancestor_record_offset":3060,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12568,"parent_record_offset":12536,"ancestor_record_offset":3060,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3865,"record_offset":14608,"parent_record_offset":13968,"ancestor_record_offset":3060,"inlinee":22357,"source_line":103,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3865,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":3060,"inlinee":27550,"source_line":215,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::quat_to_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":3865,"record_offset":14012,"parent_record_offset":13968,"ancestor_record_offset":3060,"inlinee":27551,"source_line":181,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3865,"record_offset":8880,"parent_record_offset":8856,"ancestor_record_offset":3060,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$20::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3865,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":3060,"inlinee":27553,"source_line":963,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3865,"record_offset":9324,"parent_record_offset":8880,"ancestor_record_offset":3060,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3865,"record_offset":14196,"parent_record_offset":14012,"ancestor_record_offset":3060,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3865,"record_offset":14300,"parent_record_offset":13968,"ancestor_record_offset":3060,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$44::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3865,"record_offset":14156,"parent_record_offset":14132,"ancestor_record_offset":3060,"inlinee":27555,"source_line":1273,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3865,"record_offset":14552,"parent_record_offset":13968,"ancestor_record_offset":3060,"inlinee":27557,"source_line":1300,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3865,"record_offset":8952,"parent_record_offset":8924,"ancestor_record_offset":3060,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3865,"record_offset":8924,"parent_record_offset":8880,"ancestor_record_offset":3060,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3865,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":3060,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3865,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":3060,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":6608,"parent_record_offset":6300,"ancestor_record_offset":3060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":7244,"parent_record_offset":7184,"ancestor_record_offset":3060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":6300,"parent_record_offset":6272,"ancestor_record_offset":3060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":3060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":3060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":3060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":6380,"parent_record_offset":6348,"ancestor_record_offset":3060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":6412,"parent_record_offset":6380,"ancestor_record_offset":3060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":6348,"parent_record_offset":6300,"ancestor_record_offset":3060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":3060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3865,"record_offset":6476,"parent_record_offset":6300,"ancestor_record_offset":3060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3865,"record_offset":6536,"parent_record_offset":6476,"ancestor_record_offset":3060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":4984,"parent_record_offset":4948,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3865,"record_offset":5096,"parent_record_offset":5056,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":5260,"parent_record_offset":5056,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3865,"record_offset":5528,"parent_record_offset":5260,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":3060,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":6224,"parent_record_offset":6196,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3865,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":3060,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3865,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3865,"record_offset":5156,"parent_record_offset":5124,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3865,"record_offset":5056,"parent_record_offset":5020,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3865,"record_offset":5020,"parent_record_offset":4984,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":3865,"record_offset":18336,"parent_record_offset":18308,"ancestor_record_offset":3060,"inlinee":89865,"source_line":269,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":3865,"record_offset":12420,"parent_record_offset":12388,"ancestor_record_offset":3060,"inlinee":111894,"source_line":312,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12756,"parent_record_offset":12720,"ancestor_record_offset":3060,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":3060,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":10844,"parent_record_offset":10772,"ancestor_record_offset":3060,"inlinee":21741,"source_line":944,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":10332,"parent_record_offset":10288,"ancestor_record_offset":3060,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":8700,"parent_record_offset":8576,"ancestor_record_offset":3060,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8544,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8516,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8328,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8356,"parent_record_offset":8328,"ancestor_record_offset":3060,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":3060,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":8424,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":11100,"parent_record_offset":10772,"ancestor_record_offset":3060,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12680,"parent_record_offset":12628,"ancestor_record_offset":3060,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":3060,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3865,"record_offset":12720,"parent_record_offset":12420,"ancestor_record_offset":3060,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F06
// 首条位置: {"module":2756,"inlinee":89862}
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
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F17
// 首条位置: {"module":2761,"inlinee":89879}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":2761,"record_offset":1444,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":15693,"source_line":1157,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_direct] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":2761,"record_offset":2740,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":21868,"source_line":223,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":1660,"parent_record_offset":1568,"ancestor_record_offset":1404,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":1536,"parent_record_offset":1476,"ancestor_record_offset":1404,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":1508,"parent_record_offset":1476,"ancestor_record_offset":1404,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":2761,"record_offset":1816,"parent_record_offset":1444,"ancestor_record_offset":1404,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":2761,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":1404,"inlinee":15695,"source_line":803,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":2761,"record_offset":1568,"parent_record_offset":1444,"ancestor_record_offset":1404,"inlinee":15696,"source_line":1313,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":2761,"record_offset":1692,"parent_record_offset":1444,"ancestor_record_offset":1404,"inlinee":15698,"source_line":1823,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F09
// 首条位置: {"module":2756,"inlinee":89865}
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
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B514
// 首条位置: {"module":3865,"inlinee":111892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":11724,"parent_record_offset":10772,"ancestor_record_offset":10772,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":11392,"parent_record_offset":10772,"ancestor_record_offset":10772,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":10844,"parent_record_offset":10772,"ancestor_record_offset":10772,"inlinee":21741,"source_line":944,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":11100,"parent_record_offset":10772,"ancestor_record_offset":10772,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3865,"record_offset":11580,"parent_record_offset":11552,"ancestor_record_offset":10772,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3865,"record_offset":11640,"parent_record_offset":11608,"ancestor_record_offset":10772,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3865,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":10772,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":12388,"parent_record_offset":11756,"ancestor_record_offset":10772,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3865,"record_offset":11756,"parent_record_offset":11724,"ancestor_record_offset":10772,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3865,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":10772,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":11332,"parent_record_offset":11100,"ancestor_record_offset":10772,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":11360,"parent_record_offset":11100,"ancestor_record_offset":10772,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3865,"record_offset":11304,"parent_record_offset":11100,"ancestor_record_offset":10772,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":10948,"parent_record_offset":10912,"ancestor_record_offset":10772,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":12536,"parent_record_offset":12508,"ancestor_record_offset":10772,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3865,"record_offset":11676,"parent_record_offset":11524,"ancestor_record_offset":10772,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3865,"record_offset":12224,"parent_record_offset":12200,"ancestor_record_offset":10772,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3865,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":10772,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3865,"record_offset":11148,"parent_record_offset":11100,"ancestor_record_offset":10772,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3865,"record_offset":11176,"parent_record_offset":11148,"ancestor_record_offset":10772,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3865,"record_offset":12876,"parent_record_offset":12420,"ancestor_record_offset":10772,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12628,"parent_record_offset":12508,"ancestor_record_offset":10772,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12508,"parent_record_offset":12472,"ancestor_record_offset":10772,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12472,"parent_record_offset":12420,"ancestor_record_offset":10772,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12568,"parent_record_offset":12536,"ancestor_record_offset":10772,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3865,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":10772,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3865,"record_offset":11816,"parent_record_offset":11792,"ancestor_record_offset":10772,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":12176,"parent_record_offset":11868,"ancestor_record_offset":10772,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11524,"parent_record_offset":11464,"ancestor_record_offset":10772,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11868,"parent_record_offset":11840,"ancestor_record_offset":10772,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10772,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11496,"parent_record_offset":11464,"ancestor_record_offset":10772,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10772,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":11948,"parent_record_offset":11916,"ancestor_record_offset":10772,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":11980,"parent_record_offset":11948,"ancestor_record_offset":10772,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3865,"record_offset":11916,"parent_record_offset":11868,"ancestor_record_offset":10772,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3865,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":10772,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3865,"record_offset":11552,"parent_record_offset":11524,"ancestor_record_offset":10772,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3865,"record_offset":11608,"parent_record_offset":11552,"ancestor_record_offset":10772,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3865,"record_offset":11792,"parent_record_offset":11756,"ancestor_record_offset":10772,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3865,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":10772,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/chimney_wall.rs
//     evidence={"module":3865,"record_offset":12420,"parent_record_offset":12388,"ancestor_record_offset":10772,"inlinee":111894,"source_line":312,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12756,"parent_record_offset":12720,"ancestor_record_offset":10772,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":10772,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3865,"record_offset":11212,"parent_record_offset":11100,"ancestor_record_offset":10772,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":11068,"parent_record_offset":10844,"ancestor_record_offset":10772,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":11040,"parent_record_offset":10844,"ancestor_record_offset":10772,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":10884,"parent_record_offset":10844,"ancestor_record_offset":10772,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10772,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":10772,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3865,"record_offset":10984,"parent_record_offset":10844,"ancestor_record_offset":10772,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12680,"parent_record_offset":12628,"ancestor_record_offset":10772,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":10772,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3865,"record_offset":12720,"parent_record_offset":12420,"ancestor_record_offset":10772,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 312
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B516
// 首条位置: {"module":3865,"inlinee":111894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3865,"record_offset":12876,"parent_record_offset":12420,"ancestor_record_offset":12420,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12472,"parent_record_offset":12420,"ancestor_record_offset":12420,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3865,"record_offset":12720,"parent_record_offset":12420,"ancestor_record_offset":12420,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3865,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":12420,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3865,"record_offset":12536,"parent_record_offset":12508,"ancestor_record_offset":12420,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12628,"parent_record_offset":12508,"ancestor_record_offset":12420,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12508,"parent_record_offset":12472,"ancestor_record_offset":12420,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3865,"record_offset":12568,"parent_record_offset":12536,"ancestor_record_offset":12420,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3865,"record_offset":12804,"parent_record_offset":12780,"ancestor_record_offset":12420,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12756,"parent_record_offset":12720,"ancestor_record_offset":12420,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3865,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":12420,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12680,"parent_record_offset":12628,"ancestor_record_offset":12420,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03865.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3865,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":12420,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03865.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys
// 信息 ID: 3054c59262038b6d01c39357
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 92390df93ed50eeda7d63675: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17c259f -> 0x1c65b20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1c80bc6 -> 0x1c65b20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$0
// 信息 ID: aea2964f0efc8790af337d23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$0>,alloc::alloc::Global>; module=2756, inline_record=13124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$3
// 信息 ID: 485249801b0301bda5c00d04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3 <- core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<array$<glam::f32::vec3::Vec3,3>,core::slice::iter::Iter<utils::geometry::intersect::Triangle>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3,tuple$<>,core::iter::adapters::flatten::impl$33::iter_fold::flatten::closure_env$0<array$<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::adapters::flatten::impl$35::fold::flatten::closure_env$0<core::array::iter::IntoIter<glam::f32::vec3::Vec3,3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env$3> > > > > >; module=2761, inline_record=1404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure$4
// 信息 ID: bf8013a5ceb49d452b146400
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=18336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement
// 信息 ID: bb04706304c9cefc5192f84b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=10772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_wall::is_valid_body_placement::closure$0
// 信息 ID: e480f4b39bc3db60f3222e8d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::chimney_wall::is_valid_body_placement", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys; module=3865, inline_record=12420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
