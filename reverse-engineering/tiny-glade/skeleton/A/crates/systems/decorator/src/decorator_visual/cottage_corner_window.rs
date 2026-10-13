// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 19e799c2baf31038c3a5033b260530c7119898239d546a3ed60c6d33dd1f4712
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_corner_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3858
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
// 函数符号: system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3858,"record_offset":3608,"rva":29703344,"va":5398412464}
// 编译实例 RVAs: 0x1c53cb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26769603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e83d8767fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398421342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25664153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8676578fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398413844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25667642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8c65778fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398417349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25266784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8a0757efe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398416619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=4
//     target_aliases=15 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23859386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e846ef93fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398414085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23700416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8405c96fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398413291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23698059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8756596fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398414854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23204330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e816ee9dfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398414101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23200035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8ddfe9dfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398413262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21470303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8a163b8fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398414266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21470252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8d463b8fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398414247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16505975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8892304ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398415858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16160840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8b86709ff","size":5,"soft_fail":false,"target_va":5382255968,"va":5398416803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16047125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8eb230bff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398414128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a7d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4905135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e85127b5ff","size":5,"soft_fail":false,"target_va":5393513840,"va":5398418970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4558078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8fe8c4500","size":5,"soft_fail":false,"target_va":5402978144,"va":5398420061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-5036296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8f826b3ff","size":5,"soft_fail":false,"target_va":5393378208,"va":5398414499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4558490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e89a8e4500","size":5,"soft_fail":false,"target_va":5402978688,"va":5398420193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4559191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e857914500","size":5,"soft_fail":false,"target_va":5402978880,"va":5398419684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t4559434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e84a924500","size":5,"soft_fail":false,"target_va":5402979152,"va":5398419713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4559385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e819924500","size":5,"soft_fail":false,"target_va":5402978944,"va":5398419554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8cedfffff","size":5,"soft_fail":false,"target_va":5398411104,"va":5398419341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4567084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e82cb04500","size":5,"soft_fail":false,"target_va":5402980128,"va":5398413039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8d7faffff","size":5,"soft_fail":false,"target_va":5398411296,"va":5398412612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4567962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e89ab34500","size":5,"soft_fail":false,"target_va":5402980656,"va":5398412689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24051228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8e40191fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398413799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23847030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e88a1f94fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398418017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23200997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e81bfb9dfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398413680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16480788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8ec8504ff","size":5,"soft_fail":false,"target_va":5381933120,"va":5398413903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16483710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8827a04ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398416761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22978757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e83b5fa1fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5398413728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22977350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8ba64a1fe","size":5,"soft_fail":false,"target_va":5375436640,"va":5398413985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22056305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e88f72affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398413212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22052412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8c481affe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398412967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22056453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8fb71affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398412592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16123184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8d0fa09ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398412987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4925863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e859d6b4ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398416530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4923404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8f4dfb4ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398414887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::add_lintels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t58482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e872e40000","size":5,"soft_fail":false,"target_va":5398476720,"va":5398418233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t57558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8d6e00000","size":5,"soft_fail":false,"target_va":5398476240,"va":5398418677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20455518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8a2dfc7fe","size":5,"soft_fail":false,"target_va":5377958496,"va":5398414009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20455338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e856e0c7fe","size":5,"soft_fail":false,"target_va":5377958704,"va":5398414037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16162877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8c35f09ff","size":5,"soft_fail":false,"target_va":5382255664,"va":5398418536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-16160319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e8c16909ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5398416906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21470914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398412464,"raw_hex":"e83e61b8fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398414285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3858,"record_offset":10532,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3858,"record_offset":8368,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3858,"record_offset":10980,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3858,"record_offset":11036,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3858,"record_offset":15988,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3858,"record_offset":6636,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3858,"record_offset":6608,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":8332,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3858,"record_offset":14636,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":9820,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":11488,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":12948,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":6552,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3858,"record_offset":14412,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":15840,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60165,"source_line":237,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":16240,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":26811,"source_line":237,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3858,"record_offset":11064,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":10208,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":10504,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":10040,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9984,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3858,"record_offset":15716,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":5504,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":3916,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":4492,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3858,"record_offset":7376,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3858,"record_offset":7736,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_cloth
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3858,"record_offset":15808,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":89772,"source_line":210,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3858,"record_offset":11368,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_corner_window::get_corner_info
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
//     evidence={"module":3858,"record_offset":8704,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":111851,"source_line":297,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3858,"record_offset":11204,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3858,"record_offset":14468,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3858,"record_offset":11176,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3858,"record_offset":15020,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3858,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":3608,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3858,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":3608,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3858,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":3608,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3858,"record_offset":8544,"parent_record_offset":8452,"ancestor_record_offset":3608,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3858,"record_offset":14788,"parent_record_offset":14688,"ancestor_record_offset":3608,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":14720,"parent_record_offset":14688,"ancestor_record_offset":3608,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":10604,"parent_record_offset":10572,"ancestor_record_offset":3608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":3608,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":8424,"parent_record_offset":8396,"ancestor_record_offset":3608,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":8664,"parent_record_offset":8424,"ancestor_record_offset":3608,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3858,"record_offset":14688,"parent_record_offset":14660,"ancestor_record_offset":3608,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3858,"record_offset":12768,"parent_record_offset":11524,"ancestor_record_offset":3608,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3858,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":3608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3858,"record_offset":10572,"parent_record_offset":10532,"ancestor_record_offset":3608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3858,"record_offset":8396,"parent_record_offset":8368,"ancestor_record_offset":3608,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3858,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":3608,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3858,"record_offset":8488,"parent_record_offset":8452,"ancestor_record_offset":3608,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3858,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":3608,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":3608,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":3608,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":5932,"parent_record_offset":5904,"ancestor_record_offset":3608,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":5844,"parent_record_offset":5816,"ancestor_record_offset":3608,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":5876,"parent_record_offset":5844,"ancestor_record_offset":3608,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":6048,"parent_record_offset":5876,"ancestor_record_offset":3608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3858,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3608,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3858,"record_offset":12432,"parent_record_offset":12404,"ancestor_record_offset":3608,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3858,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3858,"record_offset":6996,"parent_record_offset":6964,"ancestor_record_offset":3608,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3858,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":3608,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3858,"record_offset":7060,"parent_record_offset":7024,"ancestor_record_offset":3608,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3858,"record_offset":15476,"parent_record_offset":15236,"ancestor_record_offset":3608,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3858,"record_offset":15384,"parent_record_offset":15328,"ancestor_record_offset":3608,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3858,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":3608,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3858,"record_offset":15620,"parent_record_offset":15504,"ancestor_record_offset":3608,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3858,"record_offset":15592,"parent_record_offset":15504,"ancestor_record_offset":3608,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3858,"record_offset":11456,"parent_record_offset":11368,"ancestor_record_offset":3608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3858,"record_offset":9940,"parent_record_offset":9916,"ancestor_record_offset":3608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3858,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":3608,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3858,"record_offset":5980,"parent_record_offset":5956,"ancestor_record_offset":3608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3858,"record_offset":14752,"parent_record_offset":14720,"ancestor_record_offset":3608,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3858,"record_offset":4128,"parent_record_offset":4032,"ancestor_record_offset":3608,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3858,"record_offset":15940,"parent_record_offset":15912,"ancestor_record_offset":3608,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3858,"record_offset":16404,"parent_record_offset":16264,"ancestor_record_offset":3608,"inlinee":13340,"source_line":1703,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3858,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":3608,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":3608,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":3608,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":15748,"parent_record_offset":15716,"ancestor_record_offset":3608,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":3608,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3858,"record_offset":7340,"parent_record_offset":7308,"ancestor_record_offset":3608,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":5740,"parent_record_offset":5712,"ancestor_record_offset":3608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":11400,"parent_record_offset":11368,"ancestor_record_offset":3608,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":3608,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":5088,"parent_record_offset":4628,"ancestor_record_offset":3608,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":5788,"parent_record_offset":5612,"ancestor_record_offset":3608,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":8812,"parent_record_offset":8704,"ancestor_record_offset":3608,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":9884,"parent_record_offset":9856,"ancestor_record_offset":3608,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3858,"record_offset":4440,"parent_record_offset":4412,"ancestor_record_offset":3608,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3858,"record_offset":6112,"parent_record_offset":5816,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3858,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":3608,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3858,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":3608,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3858,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":3608,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3858,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":3608,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3858,"record_offset":5048,"parent_record_offset":5016,"ancestor_record_offset":3608,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3858,"record_offset":8620,"parent_record_offset":8592,"ancestor_record_offset":3608,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3858,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3858,"record_offset":6512,"parent_record_offset":5576,"ancestor_record_offset":3608,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3858,"record_offset":4064,"parent_record_offset":4032,"ancestor_record_offset":3608,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3858,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":3608,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3858,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":3608,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3858,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":3608,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3858,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":3608,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3858,"record_offset":4792,"parent_record_offset":4684,"ancestor_record_offset":3608,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3858,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3608,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3858,"record_offset":4524,"parent_record_offset":4492,"ancestor_record_offset":3608,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3858,"record_offset":4656,"parent_record_offset":4628,"ancestor_record_offset":3608,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3858,"record_offset":4628,"parent_record_offset":4596,"ancestor_record_offset":3608,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3858,"record_offset":4572,"parent_record_offset":4548,"ancestor_record_offset":3608,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3858,"record_offset":4596,"parent_record_offset":4572,"ancestor_record_offset":3608,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3858,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":3608,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3858,"record_offset":5120,"parent_record_offset":4596,"ancestor_record_offset":3608,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":4228,"parent_record_offset":4196,"ancestor_record_offset":3608,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":4388,"parent_record_offset":4228,"ancestor_record_offset":3608,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3608,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":3608,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3608,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":4004,"parent_record_offset":3972,"ancestor_record_offset":3608,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3858,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":3608,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3858,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":3608,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3858,"record_offset":4932,"parent_record_offset":4684,"ancestor_record_offset":3608,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3858,"record_offset":4904,"parent_record_offset":4684,"ancestor_record_offset":3608,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3858,"record_offset":4736,"parent_record_offset":4684,"ancestor_record_offset":3608,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3858,"record_offset":12796,"parent_record_offset":12768,"ancestor_record_offset":3608,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3858,"record_offset":17456,"parent_record_offset":17420,"ancestor_record_offset":3608,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3858,"record_offset":12820,"parent_record_offset":12796,"ancestor_record_offset":3608,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3858,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":3608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3858,"record_offset":17512,"parent_record_offset":17420,"ancestor_record_offset":3608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":11524,"parent_record_offset":11488,"ancestor_record_offset":3608,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":9856,"parent_record_offset":9820,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":12988,"parent_record_offset":12948,"ancestor_record_offset":3608,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":12460,"parent_record_offset":12432,"ancestor_record_offset":3608,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":3608,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":9916,"parent_record_offset":9884,"ancestor_record_offset":3608,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3858,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":3608,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":15864,"parent_record_offset":15840,"ancestor_record_offset":3608,"inlinee":60166,"source_line":143,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":3608,"inlinee":39816,"source_line":161,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":15888,"parent_record_offset":15864,"ancestor_record_offset":3608,"inlinee":60167,"source_line":122,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":15912,"parent_record_offset":15888,"ancestor_record_offset":3608,"inlinee":26812,"source_line":128,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":3858,"record_offset":16376,"parent_record_offset":16264,"ancestor_record_offset":3608,"inlinee":39817,"source_line":214,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3858,"record_offset":11088,"parent_record_offset":11064,"ancestor_record_offset":3608,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3858,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":3608,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9044,"parent_record_offset":9012,"ancestor_record_offset":3608,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":3608,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8988,"parent_record_offset":8960,"ancestor_record_offset":3608,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9788,"parent_record_offset":8704,"ancestor_record_offset":3608,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8960,"parent_record_offset":8704,"ancestor_record_offset":3608,"inlinee":45876,"source_line":465,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8904,"parent_record_offset":8704,"ancestor_record_offset":3608,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9336,"parent_record_offset":9176,"ancestor_record_offset":3608,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3858,"record_offset":15328,"parent_record_offset":15304,"ancestor_record_offset":3608,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3858,"record_offset":15304,"parent_record_offset":15236,"ancestor_record_offset":3608,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3858,"record_offset":12404,"parent_record_offset":12376,"ancestor_record_offset":3608,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":3608,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":11604,"parent_record_offset":11580,"ancestor_record_offset":3608,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":3608,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":12376,"parent_record_offset":12124,"ancestor_record_offset":3608,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":6688,"parent_record_offset":6660,"ancestor_record_offset":3608,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3858,"record_offset":11636,"parent_record_offset":11604,"ancestor_record_offset":3608,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12276,"parent_record_offset":12248,"ancestor_record_offset":3608,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12248,"parent_record_offset":12124,"ancestor_record_offset":3608,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":3608,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":7096,"parent_record_offset":6748,"ancestor_record_offset":3608,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12600,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":7532,"parent_record_offset":7472,"ancestor_record_offset":3608,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":6748,"parent_record_offset":6716,"ancestor_record_offset":3608,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":3608,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":3608,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":11664,"parent_record_offset":11604,"ancestor_record_offset":3608,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":3608,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":3608,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":11716,"parent_record_offset":11688,"ancestor_record_offset":3608,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":3608,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3858,"record_offset":6832,"parent_record_offset":6796,"ancestor_record_offset":3608,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3858,"record_offset":6868,"parent_record_offset":6832,"ancestor_record_offset":3608,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3858,"record_offset":6928,"parent_record_offset":6796,"ancestor_record_offset":3608,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3858,"record_offset":6796,"parent_record_offset":6748,"ancestor_record_offset":3608,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":11772,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":6716,"parent_record_offset":6688,"ancestor_record_offset":3608,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3858,"record_offset":12124,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3858,"record_offset":11800,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3858,"record_offset":6964,"parent_record_offset":6748,"ancestor_record_offset":3608,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3858,"record_offset":11884,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3858,"record_offset":12508,"parent_record_offset":11716,"ancestor_record_offset":3608,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3858,"record_offset":7024,"parent_record_offset":6964,"ancestor_record_offset":3608,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":5540,"parent_record_offset":5504,"ancestor_record_offset":3608,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3858,"record_offset":5652,"parent_record_offset":5612,"ancestor_record_offset":3608,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":5816,"parent_record_offset":5612,"ancestor_record_offset":3608,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3858,"record_offset":6084,"parent_record_offset":5816,"ancestor_record_offset":3608,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3858,"record_offset":11556,"parent_record_offset":11524,"ancestor_record_offset":3608,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3858,"record_offset":7400,"parent_record_offset":7376,"ancestor_record_offset":3608,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3858,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3858,"record_offset":9076,"parent_record_offset":9044,"ancestor_record_offset":3608,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3858,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":3608,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3858,"record_offset":5712,"parent_record_offset":5680,"ancestor_record_offset":3608,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3858,"record_offset":5612,"parent_record_offset":5576,"ancestor_record_offset":3608,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3858,"record_offset":5576,"parent_record_offset":5540,"ancestor_record_offset":3608,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3858,"record_offset":8848,"parent_record_offset":8704,"ancestor_record_offset":3608,"inlinee":60023,"source_line":205,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_corner_window::get_corner_info
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B4EB
// 首条位置: {"module":3858,"inlinee":111851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3858,"record_offset":8812,"parent_record_offset":8704,"ancestor_record_offset":8704,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9788,"parent_record_offset":8704,"ancestor_record_offset":8704,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8960,"parent_record_offset":8704,"ancestor_record_offset":8704,"inlinee":45876,"source_line":465,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8904,"parent_record_offset":8704,"ancestor_record_offset":8704,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3858,"record_offset":8848,"parent_record_offset":8704,"ancestor_record_offset":8704,"inlinee":60023,"source_line":205,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":8704,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3858,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":8704,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9044,"parent_record_offset":9012,"ancestor_record_offset":8704,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9012,"parent_record_offset":8988,"ancestor_record_offset":8704,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":8988,"parent_record_offset":8960,"ancestor_record_offset":8704,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3858,"record_offset":9336,"parent_record_offset":9176,"ancestor_record_offset":8704,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03858.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3858,"record_offset":9076,"parent_record_offset":9044,"ancestor_record_offset":8704,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03858.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_corner_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_corner_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_corner_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows
// 信息 ID: 025ac66520ce41fcfa9929e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 99c78b7d2b54b0f43bad25b1: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x17be568 -> 0x1c53cb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> @ 0x1c7ea2c -> 0x1c53cb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_corner_window::get_corner_info
// 信息 ID: 63341691ccb5529c6a7d7eed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=3858, inline_record=8704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
