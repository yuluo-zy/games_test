// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/cottage_balcony_door.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 83aa3b4f07c9963bc83fe26e17e4ee9ca19e1acbea6b9c6ae5c1300179ca6733
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_balcony_door.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3854
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
// 函数符号: system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3854,"record_offset":3608,"rva":29681456,"va":5398390576}
// 编译实例 RVAs: 0x1c4e730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28774601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e837ef48fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5398392628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26744328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8f8e967fe","size":5,"soft_fail":false,"target_va":5371651744,"va":5398396067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=3
//     target_aliases=15 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23837742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8d24394fe","size":5,"soft_fail":false,"target_va":5374554704,"va":5398392441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23678516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8ccb196fe","size":5,"soft_fail":false,"target_va":5374712880,"va":5398391391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23675545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e867bd96fe","size":5,"soft_fail":false,"target_va":5374716800,"va":5398392340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23182691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e89d429efe","size":5,"soft_fail":false,"target_va":5375209776,"va":5398392462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23178135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e869549efe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398391362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21449368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e868b5b8fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5398393331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21449268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8ccb5b8fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5398393263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16483564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8147b04ff","size":5,"soft_fail":false,"target_va":5381909888,"va":5398393447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16137132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e854c409ff","size":5,"soft_fail":false,"target_va":5382255968,"va":5398393095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_balcony_door.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16025486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e872780bff","size":5,"soft_fail":false,"target_va":5382367008,"va":5398392489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-27089219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8bda662fe","size":5,"soft_fail":false,"target_va":5371305408,"va":5398394622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4583187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e813ef4500","size":5,"soft_fail":false,"target_va":5402978688,"va":5398395496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4583636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8d4f04500","size":5,"soft_fail":false,"target_va":5402978912,"va":5398395271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4583665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8f1f04500","size":5,"soft_fail":false,"target_va":5402979152,"va":5398395482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4583923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8f3f14500","size":5,"soft_fail":false,"target_va":5402978944,"va":5398395016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e828eaffff","size":5,"soft_fail":false,"target_va":5398389216,"va":5398394803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4588975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8af054600","size":5,"soft_fail":false,"target_va":5402980128,"va":5398391148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8d6faffff","size":5,"soft_fail":false,"target_va":5398389408,"va":5398390725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4589849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e819094600","size":5,"soft_fail":false,"target_va":5402980656,"va":5398390802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23821024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8208594fe","size":5,"soft_fail":false,"target_va":5374571392,"va":5398392411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23823588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e81c7b94fe","size":5,"soft_fail":false,"target_va":5374570992,"va":5398394575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23179617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e89f4e9efe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398392300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16460008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e818d704ff","size":5,"soft_fail":false,"target_va":5381933056,"va":5398393059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22034411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e815c8affe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398391318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22030521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e847d7affe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398391076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22034566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e87ac7affe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398390705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16101293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e853500aff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398391096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4903455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8e12db5ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398394122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4900894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e8e237b5ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398392377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16136576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e880c609ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5398393163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21449975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398390576,"raw_hex":"e809b3b8fe","size":5,"soft_fail":false,"target_va":5376943376,"va":5398393346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3854,"record_offset":7696,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3854,"record_offset":8436,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3854,"record_offset":8492,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3854,"record_offset":6592,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3854,"record_offset":6564,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3854,"record_offset":16016,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":14604,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":8520,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21725,"source_line":267,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":11556,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60285,"source_line":274,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":13016,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60287,"source_line":285,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":6508,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11224,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11372,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11528,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":5460,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":3872,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":4448,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":7336,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":7840,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":8684,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3854,"record_offset":14480,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3854,"record_offset":15988,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60170,"source_line":72,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3854,"record_offset":15960,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":3854,"record_offset":11168,"parent_record_offset":3608,"ancestor_record_offset":3608,"inlinee":60023,"source_line":205,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3854,"record_offset":7776,"parent_record_offset":7744,"ancestor_record_offset":3608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3854,"record_offset":12836,"parent_record_offset":11592,"ancestor_record_offset":3608,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3854,"record_offset":16040,"parent_record_offset":16016,"ancestor_record_offset":3608,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3854,"record_offset":7744,"parent_record_offset":7696,"ancestor_record_offset":3608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3854,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":3608,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":3608,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":3608,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":5888,"parent_record_offset":5860,"ancestor_record_offset":3608,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":5832,"parent_record_offset":5800,"ancestor_record_offset":3608,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":6004,"parent_record_offset":5832,"ancestor_record_offset":3608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3854,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":3608,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3854,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":3608,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3854,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":3608,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3854,"record_offset":6956,"parent_record_offset":6924,"ancestor_record_offset":3608,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3854,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":3608,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3854,"record_offset":7020,"parent_record_offset":6984,"ancestor_record_offset":3608,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3854,"record_offset":14572,"parent_record_offset":14480,"ancestor_record_offset":3608,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3854,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":3608,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3854,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":3608,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3854,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":3608,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3854,"record_offset":9040,"parent_record_offset":9016,"ancestor_record_offset":3608,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3854,"record_offset":8992,"parent_record_offset":8836,"ancestor_record_offset":3608,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":3608,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":8948,"parent_record_offset":8920,"ancestor_record_offset":3608,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":3608,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":3608,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":3608,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":3608,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":9288,"parent_record_offset":9240,"ancestor_record_offset":3608,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3854,"record_offset":9680,"parent_record_offset":9240,"ancestor_record_offset":3608,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3854,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":3608,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3854,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3608,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3854,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":3608,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3854,"record_offset":4084,"parent_record_offset":3988,"ancestor_record_offset":3608,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3854,"record_offset":9388,"parent_record_offset":9288,"ancestor_record_offset":3608,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3854,"record_offset":9356,"parent_record_offset":9288,"ancestor_record_offset":3608,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3854,"record_offset":6852,"parent_record_offset":6828,"ancestor_record_offset":3608,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3854,"record_offset":11308,"parent_record_offset":11248,"ancestor_record_offset":3608,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3854,"record_offset":11336,"parent_record_offset":11248,"ancestor_record_offset":3608,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3854,"record_offset":11280,"parent_record_offset":11248,"ancestor_record_offset":3608,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3854,"record_offset":7300,"parent_record_offset":7268,"ancestor_record_offset":3608,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":5696,"parent_record_offset":5668,"ancestor_record_offset":3608,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":14516,"parent_record_offset":14480,"ancestor_record_offset":3608,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":3608,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":5044,"parent_record_offset":4584,"ancestor_record_offset":3608,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":5744,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3854,"record_offset":8584,"parent_record_offset":8556,"ancestor_record_offset":3608,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3854,"record_offset":4396,"parent_record_offset":4368,"ancestor_record_offset":3608,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3854,"record_offset":6068,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3854,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":3608,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3854,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":3608,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3854,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3608,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3854,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":3608,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3854,"record_offset":5004,"parent_record_offset":4972,"ancestor_record_offset":3608,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3854,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":3608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3854,"record_offset":6468,"parent_record_offset":5532,"ancestor_record_offset":3608,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3854,"record_offset":4020,"parent_record_offset":3988,"ancestor_record_offset":3608,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3854,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3608,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3854,"record_offset":4772,"parent_record_offset":4748,"ancestor_record_offset":3608,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3854,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":3608,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3854,"record_offset":4748,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3854,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":3608,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3854,"record_offset":4480,"parent_record_offset":4448,"ancestor_record_offset":3608,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3854,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":3608,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3854,"record_offset":4584,"parent_record_offset":4552,"ancestor_record_offset":3608,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3854,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3608,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3854,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":3608,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3854,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3608,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3854,"record_offset":5076,"parent_record_offset":4552,"ancestor_record_offset":3608,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":4184,"parent_record_offset":4152,"ancestor_record_offset":3608,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":4344,"parent_record_offset":4184,"ancestor_record_offset":3608,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3608,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":3904,"parent_record_offset":3872,"ancestor_record_offset":3608,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":3608,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":3960,"parent_record_offset":3928,"ancestor_record_offset":3608,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3854,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3608,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3854,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":3608,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3854,"record_offset":4888,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3854,"record_offset":4860,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3854,"record_offset":4692,"parent_record_offset":4640,"ancestor_record_offset":3608,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3854,"record_offset":12864,"parent_record_offset":12836,"ancestor_record_offset":3608,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3854,"record_offset":16076,"parent_record_offset":16040,"ancestor_record_offset":3608,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3854,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":3608,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3854,"record_offset":16100,"parent_record_offset":16076,"ancestor_record_offset":3608,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3854,"record_offset":16132,"parent_record_offset":16040,"ancestor_record_offset":3608,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":11592,"parent_record_offset":11556,"ancestor_record_offset":3608,"inlinee":60286,"source_line":225,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":8556,"parent_record_offset":8520,"ancestor_record_offset":3608,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":13056,"parent_record_offset":13016,"ancestor_record_offset":3608,"inlinee":60288,"source_line":234,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":12528,"parent_record_offset":12500,"ancestor_record_offset":3608,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":3608,"inlinee":60292,"source_line":228,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":3608,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3854,"record_offset":14412,"parent_record_offset":14388,"ancestor_record_offset":3608,"inlinee":60290,"source_line":237,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3854,"record_offset":9016,"parent_record_offset":8992,"ancestor_record_offset":3608,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3854,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":3608,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11452,"parent_record_offset":11420,"ancestor_record_offset":3608,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":3608,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":3608,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3854,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":3608,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3854,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":3608,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":8752,"parent_record_offset":8716,"ancestor_record_offset":3608,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":3608,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":3608,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":10900,"parent_record_offset":10676,"ancestor_record_offset":3608,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":3608,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3854,"record_offset":8812,"parent_record_offset":8776,"ancestor_record_offset":3608,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10812,"parent_record_offset":10788,"ancestor_record_offset":3608,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10788,"parent_record_offset":10676,"ancestor_record_offset":3608,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":3608,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":7056,"parent_record_offset":6704,"ancestor_record_offset":3608,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":11108,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":7492,"parent_record_offset":7432,"ancestor_record_offset":3608,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":6704,"parent_record_offset":6672,"ancestor_record_offset":3608,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10704,"parent_record_offset":10676,"ancestor_record_offset":3608,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10280,"parent_record_offset":10256,"ancestor_record_offset":3608,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10256,"parent_record_offset":8776,"ancestor_record_offset":3608,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":7408,"parent_record_offset":7384,"ancestor_record_offset":3608,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":7464,"parent_record_offset":7432,"ancestor_record_offset":3608,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":3608,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":3608,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3854,"record_offset":6792,"parent_record_offset":6756,"ancestor_record_offset":3608,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3854,"record_offset":6828,"parent_record_offset":6792,"ancestor_record_offset":3608,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3854,"record_offset":6888,"parent_record_offset":6756,"ancestor_record_offset":3608,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3854,"record_offset":6756,"parent_record_offset":6704,"ancestor_record_offset":3608,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10356,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":6672,"parent_record_offset":6644,"ancestor_record_offset":3608,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3854,"record_offset":10676,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3854,"record_offset":10384,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3854,"record_offset":6924,"parent_record_offset":6704,"ancestor_record_offset":3608,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3854,"record_offset":10468,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3854,"record_offset":11016,"parent_record_offset":10304,"ancestor_record_offset":3608,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3854,"record_offset":6984,"parent_record_offset":6924,"ancestor_record_offset":3608,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":5496,"parent_record_offset":5460,"ancestor_record_offset":3608,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3854,"record_offset":5608,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":5772,"parent_record_offset":5568,"ancestor_record_offset":3608,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3854,"record_offset":6040,"parent_record_offset":5772,"ancestor_record_offset":3608,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":8716,"parent_record_offset":8684,"ancestor_record_offset":3608,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":3608,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3854,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":3608,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3854,"record_offset":11484,"parent_record_offset":11452,"ancestor_record_offset":3608,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3854,"record_offset":8868,"parent_record_offset":8836,"ancestor_record_offset":3608,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3854,"record_offset":9192,"parent_record_offset":8836,"ancestor_record_offset":3608,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3854,"record_offset":9120,"parent_record_offset":9064,"ancestor_record_offset":3608,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3854,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":3608,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3854,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":3608,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3854,"record_offset":5568,"parent_record_offset":5532,"ancestor_record_offset":3608,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03854.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3854,"record_offset":5532,"parent_record_offset":5496,"ancestor_record_offset":3608,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03854.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_balcony_door.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_balcony_door.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_balcony_door.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors
// 信息 ID: 1445b8176d276c162d0a40fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 58df2e46a12fee5bb18b6ad9: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x17c82a9 -> 0x1c4e730 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>)> @ 0x1c7ef84 -> 0x1c4e730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
