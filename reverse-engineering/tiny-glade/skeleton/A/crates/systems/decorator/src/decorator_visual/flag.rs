// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/flag.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f94916adbb1b109c4496b4e825247207749bc6bd7a3dacd9c208eaf8236af680
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flag.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3859
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
// 函数符号: system_decorator::decorator_visual::flag::generate_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3859,"record_offset":3748,"rva":29714000,"va":5398423120}
// 编译实例 RVAs: 0x1c56650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29708203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e855b03afe","size":5,"soft_fail":false,"target_va":5368720672,"va":5398428870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26783384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8685167fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398435123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26009572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e81c2073fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398434879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25680238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8922678fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398429929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25276915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e80d4e7efe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398426750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=5
//     target_aliases=15 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23870884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e85cc293fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398425583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23856310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e84afb93fe","size":5,"soft_fail":false,"target_va":5374572480,"va":5398428785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23707574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e84a4096fe","size":5,"soft_fail":false,"target_va":5374716816,"va":5398424385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23215828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e82cc19dfe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398425599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23210828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8b4d49dfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398424055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16515755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e855fd03ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398425638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16173227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8553709ff","size":5,"soft_fail":false,"target_va":5382255760,"va":5398428982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16139275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8f5bb09ff","size":5,"soft_fail":false,"target_va":5382288704,"va":5398427974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16061825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e87fea0aff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398428828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4543864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e878554500","size":5,"soft_fail":false,"target_va":5402978144,"va":5398434275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-5048009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e837f9b2ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398426404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4544142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e88e564500","size":5,"soft_fail":false,"target_va":5402978688,"va":5398434541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4544850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e852594500","size":5,"soft_fail":false,"target_va":5402978880,"va":5398434025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4544730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8da584500","size":5,"soft_fail":false,"target_va":5402979152,"va":5398434417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4545974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8b65d4500","size":5,"soft_fail":false,"target_va":5402979344,"va":5398433365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t4545404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e87c5b4500","size":5,"soft_fail":false,"target_va":5402978944,"va":5398433535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e86ef7ffff","size":5,"soft_fail":false,"target_va":5398421760,"va":5398423949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4556413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e87d864500","size":5,"soft_fail":false,"target_va":5402980128,"va":5398423710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8d7faffff","size":5,"soft_fail":false,"target_va":5398421952,"va":5398423268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4557306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8fa894500","size":5,"soft_fail":false,"target_va":5402980656,"va":5398423345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24062728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8f8d490fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398425299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23856855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e829f993fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398427842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23705385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8d74896fe","size":5,"soft_fail":false,"target_va":5374718704,"va":5398424084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23711187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e82d3296fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5398427886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23212635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8a5cd9dfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398425318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16494882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8de4e04ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398427933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22992934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8da27a1fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5398427905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23908171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8b53093fe","size":5,"soft_fail":false,"target_va":5374521216,"va":5398429382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23919994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8860293fe","size":5,"soft_fail":false,"target_va":5374509168,"va":5398429157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22066975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8e148affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398423882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22063083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e81558affe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398423638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22067109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e85b48affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398423248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16133855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e821d109ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398423658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4935653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e81bb0b4ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398426320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4933977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398423120,"raw_hex":"e8a7b6b4ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398425460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3859,"record_offset":18960,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3859,"record_offset":8128,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3859,"record_offset":8900,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3859,"record_offset":15320,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3859,"record_offset":17148,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3859,"record_offset":24352,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3859,"record_offset":6760,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3859,"record_offset":6732,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":14380,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3859,"record_offset":24380,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3859,"record_offset":15980,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":9076,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":16712,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":10544,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":6676,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3859,"record_offset":16008,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":16036,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":13152,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":89885,"source_line":862,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":12388,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":16684,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":13064,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":17260,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":5628,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":4028,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":4616,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":14988,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":7768,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":8268,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3859,"record_offset":8956,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3859,"record_offset":22808,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":89810,"source_line":114,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3859,"record_offset":12228,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":89815,"source_line":141,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3859,"record_offset":19456,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":89816,"source_line":147,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3859,"record_offset":12036,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3859,"record_offset":12008,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3859,"record_offset":14416,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3859,"record_offset":16960,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3859,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":3748,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3859,"record_offset":19128,"parent_record_offset":19104,"ancestor_record_offset":3748,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3859,"record_offset":19040,"parent_record_offset":18996,"ancestor_record_offset":3748,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3859,"record_offset":19080,"parent_record_offset":19040,"ancestor_record_offset":3748,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3859,"record_offset":18996,"parent_record_offset":18960,"ancestor_record_offset":3748,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3859,"record_offset":8204,"parent_record_offset":8172,"ancestor_record_offset":3748,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3859,"record_offset":10364,"parent_record_offset":9116,"ancestor_record_offset":3748,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3859,"record_offset":24404,"parent_record_offset":24380,"ancestor_record_offset":3748,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3859,"record_offset":8172,"parent_record_offset":8128,"ancestor_record_offset":3748,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3859,"record_offset":8924,"parent_record_offset":8900,"ancestor_record_offset":3748,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":3748,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3748,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":3748,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":5968,"parent_record_offset":5940,"ancestor_record_offset":3748,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6000,"parent_record_offset":5968,"ancestor_record_offset":3748,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6172,"parent_record_offset":6000,"ancestor_record_offset":3748,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3859,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":3748,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3859,"record_offset":10028,"parent_record_offset":10000,"ancestor_record_offset":3748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3859,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":3748,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3859,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":3748,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3859,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":3748,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3859,"record_offset":7252,"parent_record_offset":7216,"ancestor_record_offset":3748,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3859,"record_offset":12852,"parent_record_offset":12436,"ancestor_record_offset":3748,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3859,"record_offset":12760,"parent_record_offset":12704,"ancestor_record_offset":3748,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3859,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":3748,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3859,"record_offset":12564,"parent_record_offset":12480,"ancestor_record_offset":3748,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3859,"record_offset":12624,"parent_record_offset":12436,"ancestor_record_offset":3748,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3859,"record_offset":9044,"parent_record_offset":8956,"ancestor_record_offset":3748,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3859,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":3748,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3859,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":3748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3859,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":3748,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3859,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":3748,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3859,"record_offset":6876,"parent_record_offset":6848,"ancestor_record_offset":3748,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3859,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3748,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3859,"record_offset":4252,"parent_record_offset":4156,"ancestor_record_offset":3748,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3859,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":3748,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3859,"record_offset":14896,"parent_record_offset":14872,"ancestor_record_offset":3748,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3859,"record_offset":14952,"parent_record_offset":14928,"ancestor_record_offset":3748,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3859,"record_offset":13092,"parent_record_offset":13064,"ancestor_record_offset":3748,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3859,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":3748,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3859,"record_offset":6848,"parent_record_offset":6784,"ancestor_record_offset":3748,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3859,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":3748,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":6820,"parent_record_offset":6784,"ancestor_record_offset":3748,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":3748,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":7620,"parent_record_offset":6784,"ancestor_record_offset":3748,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":3748,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":5212,"parent_record_offset":4752,"ancestor_record_offset":3748,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":5912,"parent_record_offset":5736,"ancestor_record_offset":3748,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":7680,"parent_record_offset":6760,"ancestor_record_offset":3748,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3859,"record_offset":10452,"parent_record_offset":9116,"ancestor_record_offset":3748,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3859,"record_offset":4564,"parent_record_offset":4536,"ancestor_record_offset":3748,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3859,"record_offset":6236,"parent_record_offset":5940,"ancestor_record_offset":3748,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3859,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":3748,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3859,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":3748,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3859,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":3748,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3859,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":3748,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3859,"record_offset":5172,"parent_record_offset":5140,"ancestor_record_offset":3748,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3859,"record_offset":19156,"parent_record_offset":19128,"ancestor_record_offset":3748,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3859,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":3748,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3859,"record_offset":6636,"parent_record_offset":5700,"ancestor_record_offset":3748,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3859,"record_offset":4188,"parent_record_offset":4156,"ancestor_record_offset":3748,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3859,"record_offset":14480,"parent_record_offset":14416,"ancestor_record_offset":3748,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3859,"record_offset":4216,"parent_record_offset":4188,"ancestor_record_offset":3748,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3859,"record_offset":14508,"parent_record_offset":14480,"ancestor_record_offset":3748,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3859,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":3748,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3859,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":3748,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3859,"record_offset":4916,"parent_record_offset":4808,"ancestor_record_offset":3748,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3859,"record_offset":4156,"parent_record_offset":4128,"ancestor_record_offset":3748,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3859,"record_offset":4648,"parent_record_offset":4616,"ancestor_record_offset":3748,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3859,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":3748,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3859,"record_offset":4752,"parent_record_offset":4720,"ancestor_record_offset":3748,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3859,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3748,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3859,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":3748,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3859,"record_offset":4672,"parent_record_offset":4648,"ancestor_record_offset":3748,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3859,"record_offset":5244,"parent_record_offset":4720,"ancestor_record_offset":3748,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4352,"parent_record_offset":4320,"ancestor_record_offset":3748,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4512,"parent_record_offset":4352,"ancestor_record_offset":3748,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":3748,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4064,"parent_record_offset":4028,"ancestor_record_offset":3748,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":3748,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4128,"parent_record_offset":4092,"ancestor_record_offset":3748,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3859,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":3748,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3859,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":3748,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3859,"record_offset":5056,"parent_record_offset":4808,"ancestor_record_offset":3748,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3859,"record_offset":5028,"parent_record_offset":4808,"ancestor_record_offset":3748,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3859,"record_offset":4860,"parent_record_offset":4808,"ancestor_record_offset":3748,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3859,"record_offset":10392,"parent_record_offset":10364,"ancestor_record_offset":3748,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3859,"record_offset":24440,"parent_record_offset":24404,"ancestor_record_offset":3748,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3859,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":3748,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3859,"record_offset":24464,"parent_record_offset":24440,"ancestor_record_offset":3748,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3859,"record_offset":24496,"parent_record_offset":24404,"ancestor_record_offset":3748,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3859,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":3748,"inlinee":25252,"source_line":180,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":16868,"parent_record_offset":16748,"ancestor_record_offset":3748,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":9116,"parent_record_offset":9076,"ancestor_record_offset":3748,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":16748,"parent_record_offset":16712,"ancestor_record_offset":3748,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":10584,"parent_record_offset":10544,"ancestor_record_offset":3748,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":10056,"parent_record_offset":10028,"ancestor_record_offset":3748,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":10476,"parent_record_offset":10452,"ancestor_record_offset":3748,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":16808,"parent_record_offset":16776,"ancestor_record_offset":3748,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3859,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":3748,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":3748,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":3748,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3859,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":3748,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3859,"record_offset":12880,"parent_record_offset":12436,"ancestor_record_offset":3748,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":14928,"parent_record_offset":14640,"ancestor_record_offset":3748,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":14816,"parent_record_offset":14640,"ancestor_record_offset":3748,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3859,"record_offset":14872,"parent_record_offset":14640,"ancestor_record_offset":3748,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3859,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":3748,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3859,"record_offset":12480,"parent_record_offset":12436,"ancestor_record_offset":3748,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3859,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":3748,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":9172,"parent_record_offset":9148,"ancestor_record_offset":3748,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":3748,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":3748,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":9972,"parent_record_offset":9720,"ancestor_record_offset":3748,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":3748,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3859,"record_offset":9228,"parent_record_offset":9196,"ancestor_record_offset":3748,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":3748,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9844,"parent_record_offset":9720,"ancestor_record_offset":3748,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9776,"parent_record_offset":9748,"ancestor_record_offset":3748,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":7288,"parent_record_offset":6980,"ancestor_record_offset":3748,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":10196,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":7924,"parent_record_offset":7864,"ancestor_record_offset":3748,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":6980,"parent_record_offset":6952,"ancestor_record_offset":3748,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9748,"parent_record_offset":9720,"ancestor_record_offset":3748,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9280,"parent_record_offset":9256,"ancestor_record_offset":3748,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9256,"parent_record_offset":9196,"ancestor_record_offset":3748,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":3748,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":7896,"parent_record_offset":7864,"ancestor_record_offset":3748,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9308,"parent_record_offset":9280,"ancestor_record_offset":3748,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":7864,"parent_record_offset":7840,"ancestor_record_offset":3748,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3859,"record_offset":7060,"parent_record_offset":7028,"ancestor_record_offset":3748,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3859,"record_offset":7092,"parent_record_offset":7060,"ancestor_record_offset":3748,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=7
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3859,"record_offset":8524,"parent_record_offset":8396,"ancestor_record_offset":3748,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3859,"record_offset":7028,"parent_record_offset":6980,"ancestor_record_offset":3748,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9368,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":3748,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3859,"record_offset":9720,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3859,"record_offset":9396,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3859,"record_offset":7156,"parent_record_offset":6980,"ancestor_record_offset":3748,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=10
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3859,"record_offset":9480,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3859,"record_offset":10104,"parent_record_offset":9308,"ancestor_record_offset":3748,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3859,"record_offset":7216,"parent_record_offset":7156,"ancestor_record_offset":3748,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":5664,"parent_record_offset":5628,"ancestor_record_offset":3748,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3859,"record_offset":5776,"parent_record_offset":5736,"ancestor_record_offset":3748,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":5940,"parent_record_offset":5736,"ancestor_record_offset":3748,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3859,"record_offset":6208,"parent_record_offset":5940,"ancestor_record_offset":3748,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":9148,"parent_record_offset":9116,"ancestor_record_offset":3748,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":3748,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3859,"record_offset":6904,"parent_record_offset":6876,"ancestor_record_offset":3748,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3859,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":3748,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3859,"record_offset":5804,"parent_record_offset":5776,"ancestor_record_offset":3748,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3859,"record_offset":5836,"parent_record_offset":5804,"ancestor_record_offset":3748,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3859,"record_offset":5736,"parent_record_offset":5700,"ancestor_record_offset":3748,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3859,"record_offset":5700,"parent_record_offset":5664,"ancestor_record_offset":3748,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3859,"record_offset":17000,"parent_record_offset":16960,"ancestor_record_offset":3748,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":3859,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":3748,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3859,"record_offset":14452,"parent_record_offset":14416,"ancestor_record_offset":3748,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3859,"record_offset":14544,"parent_record_offset":14416,"ancestor_record_offset":3748,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3859,"record_offset":16920,"parent_record_offset":16868,"ancestor_record_offset":3748,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/03859.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":3859,"record_offset":16892,"parent_record_offset":16868,"ancestor_record_offset":3748,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/03859.jsonl.gz"}
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
// 完整结构及原始类型记录: flag.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flag.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flag.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::generate_flags
// 信息 ID: f80d6b18118214a4944938b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 380a216a45240cabd4eb5314: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x17c4954 -> 0x1c56650 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> @ 0x1c812a1 -> 0x1c56650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
