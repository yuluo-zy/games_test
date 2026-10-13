// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0cb4819bf5786f09eba7e7d130d15f290ef3e5fb0b74558386f324036851cccf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_wall_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1284, 2756, 2761, 2767, 2801, 3863
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
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"PDB_registry_path_only","version":"1.8.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
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
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3863,"record_offset":3876,"rva":29753808,"va":5398462928}
// 编译实例 RVAs: 0x1c601d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28850803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e88dc547fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398464318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28655950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8b2be4afe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398469497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26824022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8aab266fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398475761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25719487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8418d77fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398469194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25317607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e819af7dfe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398467442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23909830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e83a2a93fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398464529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23750880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8209795fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398463755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23747837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e803a395fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398464632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23254779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e805299dfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398464550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23250499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8bd399dfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398463726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21522755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8bd96b7fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398466718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21522648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e82897b7fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398466643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16555189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e84b6303ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398465072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16210447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8f1a508ff","size":5,"soft_fail":false,"target_va":5382255968,"va":5398466410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb650
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16212492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8f49d08ff","size":5,"soft_fail":false,"target_va":5382256208,"va":5398468695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16097574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8da5e0aff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398464577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a7d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4958486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8ea56b4ff","size":5,"soft_fail":false,"target_va":5393513840,"va":5398472321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a80c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4958022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8ba58b4ff","size":5,"soft_fail":false,"target_va":5393514688,"va":5398472705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b0290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4921148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8c4e8b4ff","size":5,"soft_fail":false,"target_va":5393547920,"va":5398469063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d4250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4773663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8e128b7ff","size":5,"soft_fail":false,"target_va":5393695312,"va":5398468970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t-5087883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8755db2ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398466278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4504434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e872bb4400","size":5,"soft_fail":false,"target_va":5402978688,"va":5398474249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4505135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e82fbe4400","size":5,"soft_fail":false,"target_va":5402978912,"va":5398473772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4505346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e802bf4400","size":5,"soft_fail":false,"target_va":5402979152,"va":5398473801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4505309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8ddbe4400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398473630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e812d1ffff","size":5,"soft_fail":false,"target_va":5398461408,"va":5398473417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4516620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e80ceb4400","size":5,"soft_fail":false,"target_va":5402980128,"va":5398463503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e837faffff","size":5,"soft_fail":false,"target_va":5398461600,"va":5398463076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4517498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e87aee4400","size":5,"soft_fail":false,"target_va":5402980656,"va":5398463153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24102239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8a13a90fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398464810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23897383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8d95a93fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398468370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23251530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8b6359dfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398464213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23648488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8182797fe","size":5,"soft_fail":false,"target_va":5374821136,"va":5398469619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-16533314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8beb803ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398466365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22106769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e86fadaefe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398463676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22102876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8a4bcaefe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398463431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22106917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8dbacaefe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398463056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16173648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8b03509ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398463451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4975382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8ea14b4ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398466049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4973192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8781db4ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398464675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::add_lintels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8871d0000","size":5,"soft_fail":false,"target_va":5398476720,"va":5398469156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e818150000","size":5,"soft_fail":false,"target_va":5398476240,"va":5398470835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20512975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e831ffc6fe","size":5,"soft_fail":false,"target_va":5377952848,"va":5398465818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16212825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8a79c08ff","size":5,"soft_fail":false,"target_va":5382255664,"va":5398468484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-16209937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e8efa708ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5398466524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21523366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398462928,"raw_hex":"e85a94b7fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398466737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":14872,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":18988,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3863,"record_offset":19044,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3863,"record_offset":9888,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3863,"record_offset":9944,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3863,"record_offset":21184,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3863,"record_offset":20152,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3863,"record_offset":6944,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3863,"record_offset":6916,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3863,"record_offset":16796,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3863,"record_offset":16192,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3863,"record_offset":16852,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":9724,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":10740,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":12204,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":6860,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3863,"record_offset":9972,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":15876,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60165,"source_line":237,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":19368,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":26811,"source_line":237,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3863,"record_offset":10004,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15124,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15272,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15428,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":13696,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":25315,"source_line":1069,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":14604,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":10652,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":18600,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":17768,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":3863,"record_offset":16248,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":5812,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":4224,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":4800,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":7688,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":8048,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":8660,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_cloth
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3863,"record_offset":19976,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":89772,"source_line":210,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3863,"record_offset":14692,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::is_bay_window
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":3863,"record_offset":16024,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":89822,"source_line":381,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::subdivide_wall_decorator
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/mod.rs
//     evidence={"module":3863,"record_offset":16052,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":111880,"source_line":12,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":14844,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":15512,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":14816,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3863,"record_offset":15708,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3863,"record_offset":15012,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3863,"record_offset":15040,"parent_record_offset":3876,"ancestor_record_offset":3876,"inlinee":60023,"source_line":205,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3863,"record_offset":17016,"parent_record_offset":16992,"ancestor_record_offset":3876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3863,"record_offset":16992,"parent_record_offset":16900,"ancestor_record_offset":3876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3863,"record_offset":16928,"parent_record_offset":16900,"ancestor_record_offset":3876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3863,"record_offset":14948,"parent_record_offset":14916,"ancestor_record_offset":3876,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3863,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":3876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3863,"record_offset":22132,"parent_record_offset":22096,"ancestor_record_offset":3876,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3863,"record_offset":12024,"parent_record_offset":10780,"ancestor_record_offset":3876,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3863,"record_offset":21944,"parent_record_offset":21920,"ancestor_record_offset":3876,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":14916,"parent_record_offset":14872,"ancestor_record_offset":3876,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":16820,"parent_record_offset":16796,"ancestor_record_offset":3876,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3863,"record_offset":19680,"parent_record_offset":19656,"ancestor_record_offset":3876,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3863,"record_offset":19728,"parent_record_offset":19704,"ancestor_record_offset":3876,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3863,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":3876,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6212,"parent_record_offset":6184,"ancestor_record_offset":3876,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":3876,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6240,"parent_record_offset":6212,"ancestor_record_offset":3876,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6152,"parent_record_offset":6124,"ancestor_record_offset":3876,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":3876,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6356,"parent_record_offset":6184,"ancestor_record_offset":3876,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3863,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":3876,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3863,"record_offset":9504,"parent_record_offset":9480,"ancestor_record_offset":3876,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3863,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":3876,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3863,"record_offset":7308,"parent_record_offset":7276,"ancestor_record_offset":3876,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3863,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":3876,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3863,"record_offset":7372,"parent_record_offset":7336,"ancestor_record_offset":3876,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":10444,"parent_record_offset":10028,"ancestor_record_offset":3876,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":10352,"parent_record_offset":10296,"ancestor_record_offset":3876,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":3876,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":10156,"parent_record_offset":10072,"ancestor_record_offset":3876,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":10216,"parent_record_offset":10028,"ancestor_record_offset":3876,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3863,"record_offset":14784,"parent_record_offset":14692,"ancestor_record_offset":3876,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3863,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":3876,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3863,"record_offset":8988,"parent_record_offset":8964,"ancestor_record_offset":3876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3863,"record_offset":16700,"parent_record_offset":16676,"ancestor_record_offset":3876,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3863,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":3876,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3863,"record_offset":16308,"parent_record_offset":16272,"ancestor_record_offset":3876,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3863,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":3876,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3863,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":3876,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3863,"record_offset":16676,"parent_record_offset":16272,"ancestor_record_offset":3876,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":3863,"record_offset":19012,"parent_record_offset":18988,"ancestor_record_offset":3876,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3863,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":3876,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3863,"record_offset":16956,"parent_record_offset":16928,"ancestor_record_offset":3876,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3863,"record_offset":4436,"parent_record_offset":4340,"ancestor_record_offset":3876,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3863,"record_offset":15976,"parent_record_offset":15948,"ancestor_record_offset":3876,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3863,"record_offset":20568,"parent_record_offset":20428,"ancestor_record_offset":3876,"inlinee":13340,"source_line":1703,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3863,"record_offset":7204,"parent_record_offset":7180,"ancestor_record_offset":3876,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":3876,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":14632,"parent_record_offset":14604,"ancestor_record_offset":3876,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":10680,"parent_record_offset":10652,"ancestor_record_offset":3876,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":15180,"parent_record_offset":15148,"ancestor_record_offset":3876,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3863,"record_offset":7652,"parent_record_offset":7620,"ancestor_record_offset":3876,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":3876,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":14728,"parent_record_offset":14692,"ancestor_record_offset":3876,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":3876,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":5396,"parent_record_offset":4936,"ancestor_record_offset":3876,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":6096,"parent_record_offset":5920,"ancestor_record_offset":3876,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":9788,"parent_record_offset":9760,"ancestor_record_offset":3876,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3863,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":3876,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3863,"record_offset":6420,"parent_record_offset":6124,"ancestor_record_offset":3876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3863,"record_offset":7456,"parent_record_offset":7432,"ancestor_record_offset":3876,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3863,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":3876,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":22164,"parent_record_offset":22132,"ancestor_record_offset":3876,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":3876,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":3876,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":5356,"parent_record_offset":5324,"ancestor_record_offset":3876,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3863,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":3876,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3863,"record_offset":6820,"parent_record_offset":5884,"ancestor_record_offset":3876,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3863,"record_offset":4372,"parent_record_offset":4340,"ancestor_record_offset":3876,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3863,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":3876,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3876,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":3876,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":20180,"parent_record_offset":20152,"ancestor_record_offset":3876,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":5100,"parent_record_offset":4992,"ancestor_record_offset":3876,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3863,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":3876,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3863,"record_offset":4832,"parent_record_offset":4800,"ancestor_record_offset":3876,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3863,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":3876,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3863,"record_offset":4936,"parent_record_offset":4904,"ancestor_record_offset":3876,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3863,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":3876,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3863,"record_offset":4904,"parent_record_offset":4880,"ancestor_record_offset":3876,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3863,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":3876,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3863,"record_offset":5428,"parent_record_offset":4904,"ancestor_record_offset":3876,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4536,"parent_record_offset":4504,"ancestor_record_offset":3876,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4696,"parent_record_offset":4536,"ancestor_record_offset":3876,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3876,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4256,"parent_record_offset":4224,"ancestor_record_offset":3876,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":3876,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4312,"parent_record_offset":4280,"ancestor_record_offset":3876,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3863,"record_offset":4280,"parent_record_offset":4256,"ancestor_record_offset":3876,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3863,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":3876,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3863,"record_offset":5240,"parent_record_offset":4992,"ancestor_record_offset":3876,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3863,"record_offset":5212,"parent_record_offset":4992,"ancestor_record_offset":3876,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3863,"record_offset":5044,"parent_record_offset":4992,"ancestor_record_offset":3876,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3863,"record_offset":12052,"parent_record_offset":12024,"ancestor_record_offset":3876,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3863,"record_offset":21980,"parent_record_offset":21944,"ancestor_record_offset":3876,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3863,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":3876,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3863,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":3876,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3863,"record_offset":22036,"parent_record_offset":21944,"ancestor_record_offset":3876,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":10780,"parent_record_offset":10740,"ancestor_record_offset":3876,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":9760,"parent_record_offset":9724,"ancestor_record_offset":3876,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":12244,"parent_record_offset":12204,"ancestor_record_offset":3876,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":11716,"parent_record_offset":11688,"ancestor_record_offset":3876,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":3876,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":9820,"parent_record_offset":9788,"ancestor_record_offset":3876,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":3876,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3863,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":3876,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":3876,"inlinee":60166,"source_line":143,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":19392,"parent_record_offset":19368,"ancestor_record_offset":3876,"inlinee":39816,"source_line":161,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":3876,"inlinee":60167,"source_line":122,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":3876,"inlinee":26812,"source_line":128,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3863,"record_offset":19500,"parent_record_offset":19392,"ancestor_record_offset":3876,"inlinee":39817,"source_line":214,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3863,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":3876,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3863,"record_offset":10472,"parent_record_offset":10028,"ancestor_record_offset":3876,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3863,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":3876,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3863,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":3876,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15352,"parent_record_offset":15320,"ancestor_record_offset":3876,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":3876,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15296,"parent_record_offset":15272,"ancestor_record_offset":3876,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":15148,"parent_record_offset":15124,"ancestor_record_offset":3876,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3863,"record_offset":16132,"parent_record_offset":16052,"ancestor_record_offset":3876,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3863,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":3876,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3863,"record_offset":10072,"parent_record_offset":10028,"ancestor_record_offset":3876,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3863,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":3876,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":8728,"parent_record_offset":8692,"ancestor_record_offset":3876,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":3876,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":3876,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":9456,"parent_record_offset":9232,"ancestor_record_offset":3876,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":6996,"parent_record_offset":6968,"ancestor_record_offset":3876,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3863,"record_offset":8784,"parent_record_offset":8752,"ancestor_record_offset":3876,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":3876,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9344,"parent_record_offset":9232,"ancestor_record_offset":3876,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":3876,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7408,"parent_record_offset":7056,"ancestor_record_offset":3876,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9664,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7844,"parent_record_offset":7784,"ancestor_record_offset":3876,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7056,"parent_record_offset":7024,"ancestor_record_offset":3876,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":3876,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":3876,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":8812,"parent_record_offset":8752,"ancestor_record_offset":3876,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7760,"parent_record_offset":7736,"ancestor_record_offset":3876,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7816,"parent_record_offset":7784,"ancestor_record_offset":3876,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":3876,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":3876,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3863,"record_offset":7144,"parent_record_offset":7108,"ancestor_record_offset":3876,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3863,"record_offset":7180,"parent_record_offset":7144,"ancestor_record_offset":3876,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3863,"record_offset":7240,"parent_record_offset":7108,"ancestor_record_offset":3876,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3863,"record_offset":7108,"parent_record_offset":7056,"ancestor_record_offset":3876,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":8912,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":3876,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3863,"record_offset":9232,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3863,"record_offset":8940,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3863,"record_offset":7276,"parent_record_offset":7056,"ancestor_record_offset":3876,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3863,"record_offset":9024,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3863,"record_offset":9572,"parent_record_offset":8860,"ancestor_record_offset":3876,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3863,"record_offset":7336,"parent_record_offset":7276,"ancestor_record_offset":3876,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3863,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":3876,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":5848,"parent_record_offset":5812,"ancestor_record_offset":3876,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3863,"record_offset":5960,"parent_record_offset":5920,"ancestor_record_offset":3876,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":6124,"parent_record_offset":5920,"ancestor_record_offset":3876,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3863,"record_offset":6392,"parent_record_offset":6124,"ancestor_record_offset":3876,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":8692,"parent_record_offset":8660,"ancestor_record_offset":3876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":3876,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3863,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":3876,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3863,"record_offset":15384,"parent_record_offset":15352,"ancestor_record_offset":3876,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3863,"record_offset":5988,"parent_record_offset":5960,"ancestor_record_offset":3876,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3863,"record_offset":6020,"parent_record_offset":5988,"ancestor_record_offset":3876,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3863,"record_offset":5920,"parent_record_offset":5884,"ancestor_record_offset":3876,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3863,"record_offset":5884,"parent_record_offset":5848,"ancestor_record_offset":3876,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
//     evidence={"module":3863,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":3876,"inlinee":60255,"source_line":283,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::utils::subdivide_wall_decorator::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/mod.rs
//     evidence={"module":3863,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":3876,"inlinee":60253,"source_line":26,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3863,"record_offset":16096,"parent_record_offset":16052,"ancestor_record_offset":3876,"inlinee":60026,"source_line":231,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB5F
// 首条位置: {"module":1284,"inlinee":60255}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F0C
// 首条位置: {"module":2756,"inlinee":89868}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2756,"record_offset":29544,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29356,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29572,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29268,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29660,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2756,"record_offset":29508,"parent_record_offset":29380,"ancestor_record_offset":29232,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2756,"record_offset":29600,"parent_record_offset":29572,"ancestor_record_offset":29232,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2756,"record_offset":29688,"parent_record_offset":29660,"ancestor_record_offset":29232,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2756,"record_offset":29296,"parent_record_offset":29268,"ancestor_record_offset":29232,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29440,"parent_record_offset":29408,"ancestor_record_offset":29232,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29408,"parent_record_offset":29380,"ancestor_record_offset":29232,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2756,"record_offset":29380,"parent_record_offset":29356,"ancestor_record_offset":29232,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2756,"record_offset":29472,"parent_record_offset":29440,"ancestor_record_offset":29232,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 记录形式: compiled_procedure; instances=1
// 源码行: 404
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3863,"record_offset":22264,"rva":29767120,"va":5398476240}
// 编译实例 RVAs: 0x1c635d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417b0a00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4926684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e824d3b4ff","size":5,"soft_fail":false,"target_va":5393549824,"va":5398476503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4502345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e849b34400","size":5,"soft_fail":false,"target_va":5402978880,"va":5398476530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4502581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e835b44400","size":5,"soft_fail":false,"target_va":5402979152,"va":5398476566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4502312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e828b34400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398476627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23908420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e8bc2f93fe","size":5,"soft_fail":false,"target_va":5374567936,"va":5398476351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13906726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e8dacc2bff","size":5,"soft_fail":false,"target_va":5384569664,"va":5398476385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13907048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476240,"raw_hex":"e898cb2bff","size":5,"soft_fail":false,"target_va":5384569328,"va":5398476371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3863,"record_offset":23020,"parent_record_offset":22264,"ancestor_record_offset":22264,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3863,"record_offset":22936,"parent_record_offset":22264,"ancestor_record_offset":22264,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3863,"record_offset":22508,"parent_record_offset":22264,"ancestor_record_offset":22264,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":22436,"parent_record_offset":22264,"ancestor_record_offset":22264,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":22960,"parent_record_offset":22936,"ancestor_record_offset":22264,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3863,"record_offset":22636,"parent_record_offset":22612,"ancestor_record_offset":22264,"inlinee":8607,"source_line":60,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3863,"record_offset":22612,"parent_record_offset":22588,"ancestor_record_offset":22264,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3863,"record_offset":22588,"parent_record_offset":22556,"ancestor_record_offset":22264,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3863,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":22264,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":3863,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":22264,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3863,"record_offset":22984,"parent_record_offset":22960,"ancestor_record_offset":22264,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3863,"record_offset":22708,"parent_record_offset":22684,"ancestor_record_offset":22264,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":22684,"parent_record_offset":22660,"ancestor_record_offset":22264,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":22660,"parent_record_offset":22636,"ancestor_record_offset":22264,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F1B
// 首条位置: {"module":2761,"inlinee":89883}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2761,"record_offset":13516,"parent_record_offset":13476,"ancestor_record_offset":13476,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2761,"record_offset":14464,"parent_record_offset":13476,"ancestor_record_offset":13476,"inlinee":89885,"source_line":862,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2761,"record_offset":14376,"parent_record_offset":13476,"ancestor_record_offset":13476,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2761,"record_offset":14288,"parent_record_offset":13516,"ancestor_record_offset":13476,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":14228,"parent_record_offset":13544,"ancestor_record_offset":13476,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":14136,"parent_record_offset":14080,"ancestor_record_offset":13476,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":13592,"parent_record_offset":13544,"ancestor_record_offset":13476,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":13656,"parent_record_offset":13620,"ancestor_record_offset":13476,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2761,"record_offset":13768,"parent_record_offset":13620,"ancestor_record_offset":13476,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2761,"record_offset":14404,"parent_record_offset":14376,"ancestor_record_offset":13476,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2761,"record_offset":14488,"parent_record_offset":14464,"ancestor_record_offset":13476,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":2761,"record_offset":13544,"parent_record_offset":13516,"ancestor_record_offset":13476,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2761,"record_offset":13620,"parent_record_offset":13544,"ancestor_record_offset":13476,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2761,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13476,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2761,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":13476,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2761,"record_offset":13884,"parent_record_offset":13856,"ancestor_record_offset":13476,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":2761,"record_offset":13856,"parent_record_offset":13544,"ancestor_record_offset":13476,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 记录形式: compiled_procedure; instances=1
// 源码行: 444
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3863,"record_offset":23080,"rva":29767600,"va":5398476720}
// 编译实例 RVAs: 0x1c637b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23904439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e8493f93fe","size":5,"soft_fail":false,"target_va":5374572480,"va":5398476914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23267156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e8acf89cfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398476927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16109948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e8842e0aff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398476951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-27171608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e8e86461fe","size":5,"soft_fail":false,"target_va":5371305408,"va":5398477011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4499728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e810a94400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398479211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23905872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e8b03993fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398476859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16219297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398476720,"raw_hex":"e85f8308ff","size":5,"soft_fail":false,"target_va":5382257600,"va":5398476892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":23488,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3863,"record_offset":23624,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":23268,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3863,"record_offset":23764,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":25396,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3863,"record_offset":24380,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":24552,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":60171,"source_line":82,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":24496,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":89809,"source_line":109,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":24524,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3863,"record_offset":24468,"parent_record_offset":23080,"ancestor_record_offset":23080,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3863,"record_offset":23560,"parent_record_offset":23528,"ancestor_record_offset":23080,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3863,"record_offset":23528,"parent_record_offset":23488,"ancestor_record_offset":23080,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":24204,"parent_record_offset":23788,"ancestor_record_offset":23080,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":24112,"parent_record_offset":24056,"ancestor_record_offset":23080,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":23884,"parent_record_offset":23860,"ancestor_record_offset":23080,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":23916,"parent_record_offset":23832,"ancestor_record_offset":23080,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3863,"record_offset":23976,"parent_record_offset":23788,"ancestor_record_offset":23080,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3863,"record_offset":23388,"parent_record_offset":23364,"ancestor_record_offset":23080,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":25424,"parent_record_offset":25396,"ancestor_record_offset":23080,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3863,"record_offset":24408,"parent_record_offset":24380,"ancestor_record_offset":23080,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3863,"record_offset":23332,"parent_record_offset":23304,"ancestor_record_offset":23080,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3863,"record_offset":23716,"parent_record_offset":23660,"ancestor_record_offset":23080,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3863,"record_offset":23592,"parent_record_offset":23488,"ancestor_record_offset":23080,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3863,"record_offset":23660,"parent_record_offset":23624,"ancestor_record_offset":23080,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":23424,"parent_record_offset":23304,"ancestor_record_offset":23080,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":23304,"parent_record_offset":23268,"ancestor_record_offset":23080,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3863,"record_offset":23364,"parent_record_offset":23332,"ancestor_record_offset":23080,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3863,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":23080,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3863,"record_offset":24232,"parent_record_offset":23788,"ancestor_record_offset":23080,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3863,"record_offset":23860,"parent_record_offset":23832,"ancestor_record_offset":23080,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3863,"record_offset":23832,"parent_record_offset":23788,"ancestor_record_offset":23080,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03863.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3863,"record_offset":23448,"parent_record_offset":23424,"ancestor_record_offset":23080,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03863.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_wall_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_wall_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_wall_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec2::Vec2)
// 候选参数类型: ["glam::f32::vec2::Vec2"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 信息 ID: 8940b2710ccc657f245f3849
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c55339 -> 0x1c637b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c61a24 -> 0x1c637b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 信息 ID: 908d1060813b4f8f36f2fdd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c554f5 -> 0x1c635d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c6256a -> 0x1c635d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c620b3 -> 0x1c635d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 信息 ID: 0962eb47a51b4b3a3755ca34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$19::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; module=2761, inline_record=13476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 信息 ID: eca9ab5c59fd1dfcd4696d07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17cc8ac -> 0x1c601d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x17ec82f -> 0x1c601d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 信息 ID: a5b9d061670b9308e2ecba86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=28884; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=30664; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=2767, inline_record=7900; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=2767, inline_record=9008; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=3863, inline_record=16636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$1
// 信息 ID: 0f8c7e6f15466d74dde3b1bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=29232; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=2756, inline_record=30840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
