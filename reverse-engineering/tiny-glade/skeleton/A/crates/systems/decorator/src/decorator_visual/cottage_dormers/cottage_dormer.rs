// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4db9026797f761e0a8410f3162dd3cc9d8e3b4f277f7fc056fddf4a8a1f2325c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_dormer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 380, 1274, 2749, 4378
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"PDB_registry_path_only","version":"1.8.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
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
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x54A5
// 首条位置: {"module":380,"inlinee":21669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1274,"record_offset":3064,"rva":13582640,"va":5382291760}
// 编译实例 RVAs: 0xcf4130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12484760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8687f41ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382298307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10600180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e80c415eff","size":5,"soft_fail":false,"target_va":5371700720,"va":5382300895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9545061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e89b5a6eff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382294752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9543488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8c0606eff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382293195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=1
//     target_aliases=53 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9147866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8266a74ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5382297701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7579706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8c6578cff","size":5,"soft_fail":false,"target_va":5374712880,"va":5382292581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7576662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8aa638cff","size":5,"soft_fail":false,"target_va":5374716800,"va":5382293457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7079331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e85dfa93ff","size":5,"soft_fail":false,"target_va":5375213232,"va":5382292558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6526841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e887689cff","size":5,"soft_fail":false,"target_va":5375766912,"va":5382293748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5350433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8df5baeff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382294396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf2e10
//     owner=unresolved_namespace; count=9
//     target_aliases=3 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e81ee4ffff","size":5,"soft_fail":false,"target_va":5382286864,"va":5382293997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417a7d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11215102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8fe20ab00","size":5,"soft_fail":false,"target_va":5393513840,"va":5382298733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21300289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e841044501","size":5,"soft_fail":false,"target_va":5403594144,"va":5382293850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20678587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8bb873b01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382299552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t11081233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e81116a900","size":5,"soft_fail":false,"target_va":5393378400,"va":5382297162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20678344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8c8863b01","size":5,"soft_fail":false,"target_va":5402978688,"va":5382300339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t20679688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8088c3b01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382299459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t20680184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8f88d3b01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382299155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20679407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8ef8a3b01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382299532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e889deffff","size":5,"soft_fail":false,"target_va":5382290480,"va":5382299042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20687788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8acab3b01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382292335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e827fbffff","size":5,"soft_fail":false,"target_va":5382290672,"va":5382291908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20688666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e81aaf3b01","size":5,"soft_fail":false,"target_va":5402980656,"va":5382291985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6529569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8df5d9cff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382293116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7930813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e843fc86ff","size":5,"soft_fail":false,"target_va":5374362576,"va":5382293384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7080638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e842f593ff","size":5,"soft_fail":false,"target_va":5375212688,"va":5382293321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7477493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e80be78dff","size":5,"soft_fail":false,"target_va":5374821136,"va":5382298624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6856852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e86c5f97ff","size":5,"soft_fail":false,"target_va":5375436032,"va":5382292879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6570016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8e0bf9bff","size":5,"soft_fail":false,"target_va":5375723888,"va":5382293899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6568972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8f4c39bff","size":5,"soft_fail":false,"target_va":5375724800,"va":5382293767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6569049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8a7c39bff","size":5,"soft_fail":false,"target_va":5375724768,"va":5382293812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5935601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e80f6ea5ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5382292508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5931708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8447da5ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5382292263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5935749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e87b6da5ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5382291888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e850f6ffff","size":5,"soft_fail":false,"target_va":5382289808,"va":5382292283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11197033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e869daaa00","size":5,"soft_fail":false,"target_va":5393490672,"va":5382293634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11197983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e81fdeaa00","size":5,"soft_fail":false,"target_va":5393491488,"va":5382293500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11211040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e82011ab00","size":5,"soft_fail":false,"target_va":5393504608,"va":5382293563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-41009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e8cf5fffff","size":5,"soft_fail":false,"target_va":5382255664,"va":5382296668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5351412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382291760,"raw_hex":"e80c58aeff","size":5,"soft_fail":false,"target_va":5376943376,"va":5382294783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1274,"record_offset":8892,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1274,"record_offset":18588,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1274,"record_offset":8808,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1274,"record_offset":8864,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1274,"record_offset":18460,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1274,"record_offset":6072,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1274,"record_offset":6044,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":6604,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":7972,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1274,"record_offset":18376,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1274,"record_offset":5988,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":18228,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60165,"source_line":237,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":20620,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":26811,"source_line":237,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1274,"record_offset":19140,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1274,"record_offset":20564,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60173,"source_line":134,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":20384,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":4940,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":3352,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":3928,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1274,"record_offset":7612,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1274,"record_offset":8176,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":1274,"record_offset":8980,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1274,"record_offset":8772,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":21669,"source_line":31,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1274,"record_offset":9100,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60164,"source_line":170,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1274,"record_offset":18784,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60171,"source_line":82,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1274,"record_offset":18756,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1274,"record_offset":18976,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60172,"source_line":153,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":1274,"record_offset":18728,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12612,"parent_record_offset":12588,"ancestor_record_offset":3064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":3064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":3064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12556,"parent_record_offset":9100,"ancestor_record_offset":3064,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12588,"parent_record_offset":12556,"ancestor_record_offset":3064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":8108,"parent_record_offset":8020,"ancestor_record_offset":3064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1274,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":3064,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1274,"record_offset":8048,"parent_record_offset":8020,"ancestor_record_offset":3064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1274,"record_offset":18664,"parent_record_offset":18632,"ancestor_record_offset":3064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1274,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":3064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1274,"record_offset":18632,"parent_record_offset":18588,"ancestor_record_offset":3064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1274,"record_offset":9984,"parent_record_offset":9920,"ancestor_record_offset":3064,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1274,"record_offset":6868,"parent_record_offset":6724,"ancestor_record_offset":3064,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1274,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":3064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1274,"record_offset":6724,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7512,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7144,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":3064,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1274,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":3064,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1274,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":3064,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":3064,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3064,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":3064,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":3064,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7448,"parent_record_offset":7168,"ancestor_record_offset":3064,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7196,"parent_record_offset":7168,"ancestor_record_offset":3064,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7116,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1274,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":3064,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1274,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":3064,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1274,"record_offset":6932,"parent_record_offset":6900,"ancestor_record_offset":3064,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5340,"parent_record_offset":5312,"ancestor_record_offset":3064,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":3064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":3064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3064,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5312,"parent_record_offset":5280,"ancestor_record_offset":3064,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5484,"parent_record_offset":5312,"ancestor_record_offset":3064,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1274,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":3064,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":3064,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":3064,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":3064,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1274,"record_offset":6012,"parent_record_offset":5988,"ancestor_record_offset":3064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1274,"record_offset":6308,"parent_record_offset":6276,"ancestor_record_offset":3064,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1274,"record_offset":6372,"parent_record_offset":6336,"ancestor_record_offset":3064,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1274,"record_offset":19816,"parent_record_offset":19164,"ancestor_record_offset":3064,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1274,"record_offset":19696,"parent_record_offset":19640,"ancestor_record_offset":3064,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1274,"record_offset":19212,"parent_record_offset":19164,"ancestor_record_offset":3064,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1274,"record_offset":19272,"parent_record_offset":19240,"ancestor_record_offset":3064,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1274,"record_offset":19356,"parent_record_offset":19240,"ancestor_record_offset":3064,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1274,"record_offset":9816,"parent_record_offset":9664,"ancestor_record_offset":3064,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1274,"record_offset":9068,"parent_record_offset":8980,"ancestor_record_offset":3064,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1274,"record_offset":9844,"parent_record_offset":9664,"ancestor_record_offset":3064,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1274,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":3064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1274,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":3064,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1274,"record_offset":3564,"parent_record_offset":3468,"ancestor_record_offset":3064,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1274,"record_offset":18328,"parent_record_offset":18300,"ancestor_record_offset":3064,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1274,"record_offset":20784,"parent_record_offset":20644,"ancestor_record_offset":3064,"inlinee":13340,"source_line":1703,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1274,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":3064,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1274,"record_offset":6812,"parent_record_offset":6724,"ancestor_record_offset":3064,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":11648,"parent_record_offset":11504,"ancestor_record_offset":3064,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":13508,"parent_record_offset":13404,"ancestor_record_offset":3064,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":3064,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1274,"record_offset":6568,"parent_record_offset":6536,"ancestor_record_offset":3064,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":3064,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":9704,"parent_record_offset":9664,"ancestor_record_offset":3064,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":5176,"parent_record_offset":5148,"ancestor_record_offset":3064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":9012,"parent_record_offset":8980,"ancestor_record_offset":3064,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":3064,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":4524,"parent_record_offset":4064,"ancestor_record_offset":3064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":5224,"parent_record_offset":5048,"ancestor_record_offset":3064,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1274,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3064,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":5548,"parent_record_offset":5252,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1274,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":3064,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1274,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":3064,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1274,"record_offset":6432,"parent_record_offset":6408,"ancestor_record_offset":3064,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1274,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":3064,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1274,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3064,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1274,"record_offset":4484,"parent_record_offset":4452,"ancestor_record_offset":3064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":12664,"parent_record_offset":12636,"ancestor_record_offset":3064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":3064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1274,"record_offset":5948,"parent_record_offset":5012,"ancestor_record_offset":3064,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1274,"record_offset":7088,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1274,"record_offset":3500,"parent_record_offset":3468,"ancestor_record_offset":3064,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":10016,"parent_record_offset":9984,"ancestor_record_offset":3064,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":3528,"parent_record_offset":3500,"ancestor_record_offset":3064,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":10048,"parent_record_offset":10016,"ancestor_record_offset":3064,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1274,"record_offset":18400,"parent_record_offset":18376,"ancestor_record_offset":3064,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1274,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":3064,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1274,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":3064,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1274,"record_offset":18492,"parent_record_offset":18460,"ancestor_record_offset":3064,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1274,"record_offset":4228,"parent_record_offset":4120,"ancestor_record_offset":3064,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1274,"record_offset":3468,"parent_record_offset":3440,"ancestor_record_offset":3064,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1274,"record_offset":3960,"parent_record_offset":3928,"ancestor_record_offset":3064,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1274,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":3064,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1274,"record_offset":4064,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1274,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3064,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1274,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3064,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1274,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3064,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1274,"record_offset":4556,"parent_record_offset":4032,"ancestor_record_offset":3064,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":3064,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3824,"parent_record_offset":3664,"ancestor_record_offset":3064,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3064,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3384,"parent_record_offset":3352,"ancestor_record_offset":3064,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3848,"parent_record_offset":3824,"ancestor_record_offset":3064,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":3064,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1274,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":3064,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1274,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":3064,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1274,"record_offset":4368,"parent_record_offset":4120,"ancestor_record_offset":3064,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1274,"record_offset":4340,"parent_record_offset":4120,"ancestor_record_offset":3064,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1274,"record_offset":4172,"parent_record_offset":4120,"ancestor_record_offset":3064,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1274,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":3064,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1274,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":3064,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":3064,"inlinee":60166,"source_line":143,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":20644,"parent_record_offset":20620,"ancestor_record_offset":3064,"inlinee":39816,"source_line":161,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":18276,"parent_record_offset":18252,"ancestor_record_offset":3064,"inlinee":60167,"source_line":122,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":18300,"parent_record_offset":18276,"ancestor_record_offset":3064,"inlinee":26812,"source_line":128,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":1274,"record_offset":20756,"parent_record_offset":20644,"ancestor_record_offset":3064,"inlinee":39817,"source_line":214,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6900,"parent_record_offset":6680,"ancestor_record_offset":3064,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":3064,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":3064,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":3064,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1274,"record_offset":19164,"parent_record_offset":19140,"ancestor_record_offset":3064,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1274,"record_offset":19240,"parent_record_offset":19164,"ancestor_record_offset":3064,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1274,"record_offset":11448,"parent_record_offset":9100,"ancestor_record_offset":3064,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11360,"parent_record_offset":11324,"ancestor_record_offset":3064,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":3064,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":3064,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":3064,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":20588,"parent_record_offset":20564,"ancestor_record_offset":3064,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11268,"parent_record_offset":11240,"ancestor_record_offset":3064,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11152,"parent_record_offset":10868,"ancestor_record_offset":3064,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1274,"record_offset":19472,"parent_record_offset":19444,"ancestor_record_offset":3064,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":1274,"record_offset":19444,"parent_record_offset":19164,"ancestor_record_offset":3064,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1274,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":3064,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1274,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":3064,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":6408,"parent_record_offset":6172,"ancestor_record_offset":3064,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":7768,"parent_record_offset":7708,"ancestor_record_offset":3064,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":6172,"parent_record_offset":6144,"ancestor_record_offset":3064,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":3064,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":7740,"parent_record_offset":7708,"ancestor_record_offset":3064,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":3064,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1274,"record_offset":6240,"parent_record_offset":6212,"ancestor_record_offset":3064,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1274,"record_offset":8364,"parent_record_offset":8332,"ancestor_record_offset":3064,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1274,"record_offset":6212,"parent_record_offset":6172,"ancestor_record_offset":3064,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1274,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":3064,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1274,"record_offset":6276,"parent_record_offset":6172,"ancestor_record_offset":3064,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1274,"record_offset":6336,"parent_record_offset":6276,"ancestor_record_offset":3064,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1274,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":3064,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":4976,"parent_record_offset":4940,"ancestor_record_offset":3064,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1274,"record_offset":5088,"parent_record_offset":5048,"ancestor_record_offset":3064,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":5252,"parent_record_offset":5048,"ancestor_record_offset":3064,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1274,"record_offset":5520,"parent_record_offset":5252,"ancestor_record_offset":3064,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1274,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":3064,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1274,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1274,"record_offset":11396,"parent_record_offset":11360,"ancestor_record_offset":3064,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1274,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":3064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1274,"record_offset":5148,"parent_record_offset":5116,"ancestor_record_offset":3064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1274,"record_offset":5048,"parent_record_offset":5012,"ancestor_record_offset":3064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1274,"record_offset":5012,"parent_record_offset":4976,"ancestor_record_offset":3064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1274,"record_offset":6632,"parent_record_offset":6604,"ancestor_record_offset":3064,"inlinee":60161,"source_line":70,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1274,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":3064,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10912,"parent_record_offset":10868,"ancestor_record_offset":3064,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9920,"parent_record_offset":9884,"ancestor_record_offset":3064,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9884,"parent_record_offset":9100,"ancestor_record_offset":3064,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10868,"parent_record_offset":9100,"ancestor_record_offset":3064,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9956,"parent_record_offset":9920,"ancestor_record_offset":3064,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10092,"parent_record_offset":9920,"ancestor_record_offset":3064,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1274,"record_offset":9664,"parent_record_offset":9628,"ancestor_record_offset":3064,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9416,"parent_record_offset":9100,"ancestor_record_offset":3064,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":3064,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9628,"parent_record_offset":9592,"ancestor_record_offset":3064,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1274,"record_offset":13348,"parent_record_offset":13032,"ancestor_record_offset":3064,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1274,"record_offset":11240,"parent_record_offset":10868,"ancestor_record_offset":3064,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB01
// 首条位置: {"module":1274,"inlinee":60161}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1274,"record_offset":6656,"parent_record_offset":6632,"ancestor_record_offset":6632,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1274,"record_offset":6868,"parent_record_offset":6724,"ancestor_record_offset":6632,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1274,"record_offset":6724,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7512,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7144,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1274,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":6632,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1274,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":6632,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1274,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":6632,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6632,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6632,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":6632,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1274,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":6632,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7448,"parent_record_offset":7168,"ancestor_record_offset":6632,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7196,"parent_record_offset":7168,"ancestor_record_offset":6632,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1274,"record_offset":7116,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1274,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":6632,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1274,"record_offset":6932,"parent_record_offset":6900,"ancestor_record_offset":6632,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":6632,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1274,"record_offset":6812,"parent_record_offset":6724,"ancestor_record_offset":6632,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":6632,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":6632,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1274,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":6632,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1274,"record_offset":7088,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1274,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":6632,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":6632,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6900,"parent_record_offset":6680,"ancestor_record_offset":6632,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":6632,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":6632,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1274,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":6632,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1274,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":6632,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB04
// 首条位置: {"module":1274,"inlinee":60164}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12556,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1274,"record_offset":9592,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":13640,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1274,"record_offset":11448,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11480,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9884,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10868,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9416,"parent_record_offset":9100,"ancestor_record_offset":9100,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12612,"parent_record_offset":12588,"ancestor_record_offset":9100,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":9100,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":13828,"parent_record_offset":13804,"ancestor_record_offset":9100,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":12588,"parent_record_offset":12556,"ancestor_record_offset":9100,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1274,"record_offset":13804,"parent_record_offset":13712,"ancestor_record_offset":9100,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1274,"record_offset":18024,"parent_record_offset":18000,"ancestor_record_offset":9100,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1274,"record_offset":13740,"parent_record_offset":13712,"ancestor_record_offset":9100,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1274,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":9100,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1274,"record_offset":9984,"parent_record_offset":9920,"ancestor_record_offset":9100,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":9100,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":9776,"parent_record_offset":9752,"ancestor_record_offset":9100,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1274,"record_offset":9816,"parent_record_offset":9664,"ancestor_record_offset":9100,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1274,"record_offset":9844,"parent_record_offset":9664,"ancestor_record_offset":9100,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1274,"record_offset":13768,"parent_record_offset":13740,"ancestor_record_offset":9100,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":11648,"parent_record_offset":11504,"ancestor_record_offset":9100,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":13508,"parent_record_offset":13404,"ancestor_record_offset":9100,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":9100,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":9728,"parent_record_offset":9704,"ancestor_record_offset":9100,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":9704,"parent_record_offset":9664,"ancestor_record_offset":9100,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":13664,"parent_record_offset":13640,"ancestor_record_offset":9100,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1274,"record_offset":12664,"parent_record_offset":12636,"ancestor_record_offset":9100,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":10016,"parent_record_offset":9984,"ancestor_record_offset":9100,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1274,"record_offset":10048,"parent_record_offset":10016,"ancestor_record_offset":9100,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11360,"parent_record_offset":11324,"ancestor_record_offset":9100,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11324,"parent_record_offset":11296,"ancestor_record_offset":9100,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":9100,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":9100,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11268,"parent_record_offset":11240,"ancestor_record_offset":9100,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1274,"record_offset":11152,"parent_record_offset":10868,"ancestor_record_offset":9100,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1274,"record_offset":11396,"parent_record_offset":11360,"ancestor_record_offset":9100,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1274,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":9100,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10912,"parent_record_offset":10868,"ancestor_record_offset":9100,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9920,"parent_record_offset":9884,"ancestor_record_offset":9100,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":9956,"parent_record_offset":9920,"ancestor_record_offset":9100,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1274,"record_offset":10092,"parent_record_offset":9920,"ancestor_record_offset":9100,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1274,"record_offset":9664,"parent_record_offset":9628,"ancestor_record_offset":9100,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9444,"parent_record_offset":9416,"ancestor_record_offset":9100,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":9628,"parent_record_offset":9592,"ancestor_record_offset":9100,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1274,"record_offset":13348,"parent_record_offset":13032,"ancestor_record_offset":9100,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1274,"record_offset":11240,"parent_record_offset":10868,"ancestor_record_offset":9100,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1274,"record_offset":21264,"rva":13592800,"va":5382301920}
// 编译实例 RVAs: 0xcf68e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6535367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e839479cff","size":5,"soft_fail":false,"target_va":5375766912,"va":5382302274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5358194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e88e3daeff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382302189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21291772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e8fce24401","size":5,"soft_fail":false,"target_va":5403594144,"va":5382302367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20675688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e8687c3b01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382303459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6538526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e8e23a9cff","size":5,"soft_fail":false,"target_va":5375763504,"va":5382302025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6577295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e871a39bff","size":5,"soft_fail":false,"target_va":5375724992,"va":5382302282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4340340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e88cc5bdff","size":5,"soft_fail":false,"target_va":5377961984,"va":5382302319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-4346624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e800adbdff","size":5,"soft_fail":false,"target_va":5377955840,"va":5382302459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4346912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382301920,"raw_hex":"e8e0abbdff","size":5,"soft_fail":false,"target_va":5377955424,"va":5382302331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1274,"record_offset":21616,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1274,"record_offset":21560,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1274,"record_offset":21504,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::dormer_roof_height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":1274,"record_offset":21464,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":60175,"source_line":389,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":21828,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":21680,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":24314,"source_line":48,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":21940,"parent_record_offset":21264,"ancestor_record_offset":21264,"inlinee":60176,"source_line":73,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":22296,"parent_record_offset":22224,"ancestor_record_offset":21264,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1274,"record_offset":22340,"parent_record_offset":22296,"ancestor_record_offset":21264,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":21264,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1274,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":21264,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":22116,"parent_record_offset":22064,"ancestor_record_offset":21264,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1274,"record_offset":22064,"parent_record_offset":21940,"ancestor_record_offset":21264,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1274,"record_offset":21644,"parent_record_offset":21616,"ancestor_record_offset":21264,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1274,"record_offset":21776,"parent_record_offset":21748,"ancestor_record_offset":21264,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":21708,"parent_record_offset":21680,"ancestor_record_offset":21264,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01274.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1274,"record_offset":21748,"parent_record_offset":21708,"ancestor_record_offset":21264,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01274.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_dormer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_dormer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_dormer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
// 信息 ID: 1cdfb1a26e051454741cbcc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=380, inline_record=9644; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=8772; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=2749, inline_record=23556; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=2749, inline_record=28568; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=4378, inline_record=4788; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=4378, inline_record=12888; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=4378, inline_record=13740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers
// 信息 ID: bc7db02e466b691c2e000627
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4a9057a11b4413ac9febf1f6: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17ce839 -> 0xcf4130 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1c7fa4c -> 0xcf4130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
// 信息 ID: 5791f36c46f6fffc7827fa71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=6632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
// 信息 ID: 83baa0e4df4029dbada57ccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=1274, inline_record=9100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
// 信息 ID: 18f33d77d4ba9710bc657574
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501299 -> 0xcf68e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x5011b7 -> 0xcf68e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
