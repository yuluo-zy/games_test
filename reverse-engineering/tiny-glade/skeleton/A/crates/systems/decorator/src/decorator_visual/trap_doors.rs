// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/trap_doors.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7c62ae086e70cba1c8863756a592a4c61ec3fcd7dc4db63e5a3fa5554f4cd36c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: trap_doors.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3862
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"PDB_registry_path_only","version":"0.17.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::trap_doors::generate_trap_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3862,"record_offset":3064,"rva":29745376,"va":5398454496}
// 编译实例 RVAs: 0x1c5e0e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28644911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8d1e94afe","size":5,"soft_fail":false,"target_va":5369813552,"va":5398458458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26808530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e82eef66fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398460269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26034322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e86ebf72fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398459629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25706846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8a2be77fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398456537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25709716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e86cb377fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398459423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23742422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e82ab895fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398455297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23739541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e86bc395fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398456336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23242047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8c15a9dfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398455274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5136774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e886614e00","size":5,"soft_fail":false,"target_va":5403593360,"va":5398456581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t4519207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e827f54400","size":5,"soft_fail":false,"target_va":5402978144,"va":5398458932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5079540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e80c7eb2ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398457935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-20931870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8e29ac0fe","size":5,"soft_fail":false,"target_va":5377527120,"va":5398458985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4519614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8bef64400","size":5,"soft_fail":false,"target_va":5402978688,"va":5398459069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4520027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e85bf84400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398458912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e860eaffff","size":5,"soft_fail":false,"target_va":5398453216,"va":5398458747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4525059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8030c4500","size":5,"soft_fail":false,"target_va":5402980128,"va":5398455064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e82ffbffff","size":5,"soft_fail":false,"target_va":5398453408,"va":5398454636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4525938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8720f4500","size":5,"soft_fail":false,"target_va":5402980656,"va":5398454713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22692283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e845bea5fe","size":5,"soft_fail":false,"target_va":5375763552,"va":5398455830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24093553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e88f5c90fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398456124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23243364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e89c559dfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398456047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23637529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8e75197fe","size":5,"soft_fail":false,"target_va":5374821136,"va":5398458660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23019567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8d1bfa0fe","size":5,"soft_fail":false,"target_va":5375436032,"va":5398455594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23935248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8f0c692fe","size":5,"soft_fail":false,"target_va":5374521216,"va":5398456459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22098323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e86dceaefe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398455230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22094437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e89bddaefe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398454992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22098477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8d3cdaefe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398454616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16165209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8a75609ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398455012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4966958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8d235b4ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398457625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4964906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e8d63db4ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398456389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_circle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20487629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e83362c7fe","size":5,"soft_fail":false,"target_va":5377968672,"va":5398456296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20488422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e81a5fc7fe","size":5,"soft_fail":false,"target_va":5377967856,"va":5398456273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20496515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398454496,"raw_hex":"e87d3fc7fe","size":5,"soft_fail":false,"target_va":5377959696,"va":5398456206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3862,"record_offset":11764,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3862,"record_offset":7920,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3862,"record_offset":8684,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3862,"record_offset":8740,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3862,"record_offset":6024,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3862,"record_offset":5996,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":6552,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":8656,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3862,"record_offset":13320,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":3862,"record_offset":12484,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":61513,"source_line":26,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3862,"record_offset":5940,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$110::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":3862,"record_offset":8768,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":111874,"source_line":1135,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3862,"record_offset":12704,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3862,"record_offset":12676,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":4892,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":3304,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":3880,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3862,"record_offset":7560,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3862,"record_offset":8060,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3862,"record_offset":12308,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3862,"record_offset":12512,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60171,"source_line":82,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3862,"record_offset":12456,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":89809,"source_line":109,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3862,"record_offset":12428,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":9228,"parent_record_offset":9204,"ancestor_record_offset":3064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":9252,"parent_record_offset":9228,"ancestor_record_offset":3064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":11372,"parent_record_offset":11348,"ancestor_record_offset":3064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":11840,"parent_record_offset":11804,"ancestor_record_offset":3064,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":9204,"parent_record_offset":9096,"ancestor_record_offset":3064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3862,"record_offset":11348,"parent_record_offset":11264,"ancestor_record_offset":3064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3862,"record_offset":11804,"parent_record_offset":11764,"ancestor_record_offset":3064,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$56::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3862,"record_offset":8876,"parent_record_offset":8768,"ancestor_record_offset":3064,"inlinee":44592,"source_line":2086,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3862,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":3064,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3862,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":3064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3862,"record_offset":9096,"parent_record_offset":9060,"ancestor_record_offset":3064,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3862,"record_offset":9060,"parent_record_offset":9024,"ancestor_record_offset":3064,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3862,"record_offset":9324,"parent_record_offset":9060,"ancestor_record_offset":3064,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3862,"record_offset":8984,"parent_record_offset":8948,"ancestor_record_offset":3064,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3862,"record_offset":8948,"parent_record_offset":8912,"ancestor_record_offset":3064,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3862,"record_offset":8912,"parent_record_offset":8876,"ancestor_record_offset":3064,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3862,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":3064,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3862,"record_offset":13344,"parent_record_offset":13320,"ancestor_record_offset":3064,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3862,"record_offset":7964,"parent_record_offset":7920,"ancestor_record_offset":3064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3862,"record_offset":9024,"parent_record_offset":8984,"ancestor_record_offset":3064,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3862,"record_offset":6816,"parent_record_offset":6672,"ancestor_record_offset":3064,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3862,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":3064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3862,"record_offset":6672,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7460,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7092,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":3064,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3862,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":3064,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3862,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":3064,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":3064,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":3064,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":3064,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":3064,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7396,"parent_record_offset":7116,"ancestor_record_offset":3064,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":3064,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7064,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3862,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":3064,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3862,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":3064,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3862,"record_offset":9140,"parent_record_offset":9096,"ancestor_record_offset":3064,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3862,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":3064,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3862,"record_offset":9168,"parent_record_offset":9140,"ancestor_record_offset":3064,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3862,"record_offset":6880,"parent_record_offset":6848,"ancestor_record_offset":3064,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":3064,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":3064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":3064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":3064,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":3064,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":3064,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3862,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":3064,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3862,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":3064,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3862,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":3064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3862,"record_offset":6260,"parent_record_offset":6228,"ancestor_record_offset":3064,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3862,"record_offset":6324,"parent_record_offset":6288,"ancestor_record_offset":3064,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3862,"record_offset":13144,"parent_record_offset":12728,"ancestor_record_offset":3064,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3862,"record_offset":13052,"parent_record_offset":12996,"ancestor_record_offset":3064,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3862,"record_offset":12824,"parent_record_offset":12800,"ancestor_record_offset":3064,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3862,"record_offset":12856,"parent_record_offset":12772,"ancestor_record_offset":3064,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3862,"record_offset":12916,"parent_record_offset":12728,"ancestor_record_offset":3064,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3862,"record_offset":12396,"parent_record_offset":12308,"ancestor_record_offset":3064,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3862,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":3064,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3862,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":3064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3862,"record_offset":11316,"parent_record_offset":11292,"ancestor_record_offset":3064,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3862,"record_offset":3516,"parent_record_offset":3420,"ancestor_record_offset":3064,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3862,"record_offset":8272,"parent_record_offset":8248,"ancestor_record_offset":3064,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3862,"record_offset":6760,"parent_record_offset":6672,"ancestor_record_offset":3064,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3862,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":3064,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":5128,"parent_record_offset":5100,"ancestor_record_offset":3064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":12340,"parent_record_offset":12308,"ancestor_record_offset":3064,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":3064,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":4476,"parent_record_offset":4016,"ancestor_record_offset":3064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":5176,"parent_record_offset":5000,"ancestor_record_offset":3064,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":9996,"parent_record_offset":8768,"ancestor_record_offset":3064,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3862,"record_offset":3828,"parent_record_offset":3800,"ancestor_record_offset":3064,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3862,"record_offset":9364,"parent_record_offset":8984,"ancestor_record_offset":3064,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3862,"record_offset":5500,"parent_record_offset":5204,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3862,"record_offset":6988,"parent_record_offset":6956,"ancestor_record_offset":3064,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3862,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":3064,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3862,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":3064,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3862,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":3064,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3862,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":3064,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3862,"record_offset":4436,"parent_record_offset":4404,"ancestor_record_offset":3064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3862,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":3064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3862,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":3064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3862,"record_offset":5900,"parent_record_offset":4964,"ancestor_record_offset":3064,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3862,"record_offset":7036,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3862,"record_offset":3452,"parent_record_offset":3420,"ancestor_record_offset":3064,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3862,"record_offset":3480,"parent_record_offset":3452,"ancestor_record_offset":3064,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3862,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3064,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3862,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":3064,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3862,"record_offset":4180,"parent_record_offset":4072,"ancestor_record_offset":3064,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3862,"record_offset":3420,"parent_record_offset":3392,"ancestor_record_offset":3064,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3862,"record_offset":3912,"parent_record_offset":3880,"ancestor_record_offset":3064,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3862,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3064,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3862,"record_offset":4016,"parent_record_offset":3984,"ancestor_record_offset":3064,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3862,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":3064,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3862,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":3064,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3862,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":3064,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3862,"record_offset":4508,"parent_record_offset":3984,"ancestor_record_offset":3064,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3616,"parent_record_offset":3584,"ancestor_record_offset":3064,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3776,"parent_record_offset":3616,"ancestor_record_offset":3064,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3064,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":3064,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":3064,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":3064,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3862,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":3064,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3862,"record_offset":4072,"parent_record_offset":4044,"ancestor_record_offset":3064,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3862,"record_offset":4320,"parent_record_offset":4072,"ancestor_record_offset":3064,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3862,"record_offset":4292,"parent_record_offset":4072,"ancestor_record_offset":3064,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3862,"record_offset":4124,"parent_record_offset":4072,"ancestor_record_offset":3064,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3862,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":3064,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3862,"record_offset":13380,"parent_record_offset":13344,"ancestor_record_offset":3064,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3862,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":3064,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3862,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":3064,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3862,"record_offset":13436,"parent_record_offset":13344,"ancestor_record_offset":3064,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6848,"parent_record_offset":6628,"ancestor_record_offset":3064,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":3064,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":3064,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":3064,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3862,"record_offset":12728,"parent_record_offset":12704,"ancestor_record_offset":3064,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3862,"record_offset":13172,"parent_record_offset":12728,"ancestor_record_offset":3064,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3862,"record_offset":12800,"parent_record_offset":12772,"ancestor_record_offset":3064,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3862,"record_offset":12772,"parent_record_offset":12728,"ancestor_record_offset":3064,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3862,"record_offset":7608,"parent_record_offset":7584,"ancestor_record_offset":3064,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3862,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":3064,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":6360,"parent_record_offset":6124,"ancestor_record_offset":3064,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":7716,"parent_record_offset":7656,"ancestor_record_offset":3064,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":6124,"parent_record_offset":6096,"ancestor_record_offset":3064,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":7632,"parent_record_offset":7608,"ancestor_record_offset":3064,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":3064,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":3064,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3862,"record_offset":6192,"parent_record_offset":6164,"ancestor_record_offset":3064,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3862,"record_offset":8248,"parent_record_offset":8216,"ancestor_record_offset":3064,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3862,"record_offset":6164,"parent_record_offset":6124,"ancestor_record_offset":3064,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3862,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":3064,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3862,"record_offset":6228,"parent_record_offset":6124,"ancestor_record_offset":3064,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3862,"record_offset":6288,"parent_record_offset":6228,"ancestor_record_offset":3064,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3862,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":3064,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::shape::shared_shape::impl$4::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":3862,"record_offset":11104,"parent_record_offset":8768,"ancestor_record_offset":3064,"inlinee":22362,"source_line":21,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":4928,"parent_record_offset":4892,"ancestor_record_offset":3064,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3862,"record_offset":5040,"parent_record_offset":5000,"ancestor_record_offset":3064,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":5204,"parent_record_offset":5000,"ancestor_record_offset":3064,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3862,"record_offset":5472,"parent_record_offset":5204,"ancestor_record_offset":3064,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3862,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":3064,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3862,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3862,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":3064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3862,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":3064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3862,"record_offset":5000,"parent_record_offset":4964,"ancestor_record_offset":3064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3862,"record_offset":4964,"parent_record_offset":4928,"ancestor_record_offset":3064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/trap_doors.rs
//     evidence={"module":3862,"record_offset":6580,"parent_record_offset":6552,"ancestor_record_offset":3064,"inlinee":111872,"source_line":40,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B500
// 首条位置: {"module":3862,"inlinee":111872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3862,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":6580,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3862,"record_offset":6816,"parent_record_offset":6672,"ancestor_record_offset":6580,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3862,"record_offset":6672,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7460,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7092,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3862,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":6580,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3862,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":6580,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3862,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6580,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6580,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":6580,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":6580,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3862,"record_offset":7328,"parent_record_offset":7304,"ancestor_record_offset":6580,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7396,"parent_record_offset":7116,"ancestor_record_offset":6580,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":6580,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3862,"record_offset":7064,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3862,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":6580,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3862,"record_offset":6880,"parent_record_offset":6848,"ancestor_record_offset":6580,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3862,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":6580,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3862,"record_offset":6760,"parent_record_offset":6672,"ancestor_record_offset":6580,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":6580,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3862,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6580,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3862,"record_offset":6988,"parent_record_offset":6956,"ancestor_record_offset":6580,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3862,"record_offset":7036,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3862,"record_offset":6704,"parent_record_offset":6672,"ancestor_record_offset":6580,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3862,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":6580,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6848,"parent_record_offset":6628,"ancestor_record_offset":6580,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6932,"parent_record_offset":6908,"ancestor_record_offset":6580,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6956,"parent_record_offset":6932,"ancestor_record_offset":6580,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3862,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":6580,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03862.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3862,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":6580,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03862.jsonl.gz"}
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
// 完整结构及原始类型记录: trap_doors.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: trap_doors.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: trap_doors.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::trap_doors::generate_trap_doors
// 信息 ID: fd71cdd873f6f24e190c418a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6e909eca477b9135f3588367: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17cdfb6 -> 0x1c5e0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1c831b8 -> 0x1c5e0e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$0
// 信息 ID: be841aae18173bca17d4614f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::trap_doors::generate_trap_doors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=3862, inline_record=6580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
