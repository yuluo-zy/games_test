// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/chimney_roof.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3238eee4f438602a1b41352500a9c35f77bd1b0fdf80cd6dc3c0bd54ff8c7525
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: chimney_roof.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3864
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3864,"record_offset":3064,"rva":29771392,"va":5398480512}
// 编译实例 RVAs: 0x1c64680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26059092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8ac5e72fe","size":5,"soft_fail":false,"target_va":5372425312,"va":5398484399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: chimney_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25733440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8c05677fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398483131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23764868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e87c6095fe","size":5,"soft_fail":false,"target_va":5374716816,"va":5398481679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23268110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8f2f49cfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398481337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfd710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: chimney_roof.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16153480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8788409ff","size":5,"soft_fail":false,"target_va":5382330128,"va":5398483603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4494392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e838944400","size":5,"soft_fail":false,"target_va":5402978144,"va":5398483747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5103957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8ab1eb2ff","size":5,"soft_fail":false,"target_va":5393378400,"va":5398482352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4494836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8f4954400","size":5,"soft_fail":false,"target_va":5402978688,"va":5398483847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4495114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e80a974400","size":5,"soft_fail":false,"target_va":5402978880,"va":5398483761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4495106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e802974400","size":5,"soft_fail":false,"target_va":5402978944,"va":5398483833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8c9eeffff","size":5,"soft_fail":false,"target_va":5398479232,"va":5398483634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4499024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e850a64400","size":5,"soft_fail":false,"target_va":5402980128,"va":5398481099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e827fbffff","size":5,"soft_fail":false,"target_va":5398479424,"va":5398480660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4499914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8caa94400","size":5,"soft_fail":false,"target_va":5402980656,"va":5398480737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-24119443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e86df78ffe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398482014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23762661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e81b6995fe","size":5,"soft_fail":false,"target_va":5374718704,"va":5398481360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23765468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8245e95fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5398482167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23269263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e871f09cfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398481946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16550088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8387703ff","size":5,"soft_fail":false,"target_va":5381932864,"va":5398482947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23047218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8ce53a0fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5398482189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23961812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e82c5f92fe","size":5,"soft_fail":false,"target_va":5374521216,"va":5398483023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22124361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8b768aefe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398481268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22120472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8e877aefe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398481027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22124501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e82b68aefe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398480640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16191244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e8f4f008ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398481047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4992634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e886d1b3ff","size":5,"soft_fail":false,"target_va":5393490672,"va":5398483301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4990595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398480512,"raw_hex":"e87dd9b3ff","size":5,"soft_fail":false,"target_va":5393491488,"va":5398482078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3864,"record_offset":8396,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3864,"record_offset":8656,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3864,"record_offset":8256,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3864,"record_offset":7984,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3864,"record_offset":6020,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3864,"record_offset":5992,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":8164,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3864,"record_offset":5936,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3864,"record_offset":8800,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":4888,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":3288,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":3876,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3864,"record_offset":9416,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3864,"record_offset":7028,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3864,"record_offset":7388,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3864,"record_offset":8040,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60163,"source_line":214,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3864,"record_offset":8200,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::smoke
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3864,"record_offset":8228,"parent_record_offset":3064,"ancestor_record_offset":3064,"inlinee":89814,"source_line":135,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3864,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":3064,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3864,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":3064,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3864,"record_offset":8468,"parent_record_offset":8432,"ancestor_record_offset":3064,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3864,"record_offset":8500,"parent_record_offset":8468,"ancestor_record_offset":3064,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3864,"record_offset":8432,"parent_record_offset":8396,"ancestor_record_offset":3064,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3864,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":3064,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3864,"record_offset":8300,"parent_record_offset":8256,"ancestor_record_offset":3064,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3864,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":3064,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":3064,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":3064,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":3064,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":3064,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5260,"parent_record_offset":5228,"ancestor_record_offset":3064,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5432,"parent_record_offset":5260,"ancestor_record_offset":3064,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3864,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":3064,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3864,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":3064,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3864,"record_offset":6448,"parent_record_offset":6416,"ancestor_record_offset":3064,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3864,"record_offset":6512,"parent_record_offset":6476,"ancestor_record_offset":3064,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3864,"record_offset":9240,"parent_record_offset":8824,"ancestor_record_offset":3064,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3864,"record_offset":9148,"parent_record_offset":9092,"ancestor_record_offset":3064,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3864,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":3064,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3864,"record_offset":8952,"parent_record_offset":8868,"ancestor_record_offset":3064,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3864,"record_offset":9012,"parent_record_offset":8824,"ancestor_record_offset":3064,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3864,"record_offset":8132,"parent_record_offset":8040,"ancestor_record_offset":3064,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3864,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":3064,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3864,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":3064,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3864,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":3064,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3864,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":3064,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3864,"record_offset":3512,"parent_record_offset":3416,"ancestor_record_offset":3064,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3864,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":3064,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3864,"record_offset":6108,"parent_record_offset":6044,"ancestor_record_offset":3064,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3864,"record_offset":6784,"parent_record_offset":6760,"ancestor_record_offset":3064,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":6080,"parent_record_offset":6044,"ancestor_record_offset":3064,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":3064,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":6880,"parent_record_offset":6044,"ancestor_record_offset":3064,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":3064,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":4472,"parent_record_offset":4012,"ancestor_record_offset":3064,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":5172,"parent_record_offset":4996,"ancestor_record_offset":3064,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3864,"record_offset":6940,"parent_record_offset":6020,"ancestor_record_offset":3064,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3864,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":3064,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3864,"record_offset":5496,"parent_record_offset":5200,"ancestor_record_offset":3064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3864,"record_offset":6596,"parent_record_offset":6572,"ancestor_record_offset":3064,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3864,"record_offset":6572,"parent_record_offset":6548,"ancestor_record_offset":3064,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3864,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3064,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3864,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":3064,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3864,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":3064,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3864,"record_offset":8576,"parent_record_offset":8548,"ancestor_record_offset":3064,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3864,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":3064,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3864,"record_offset":5896,"parent_record_offset":4960,"ancestor_record_offset":3064,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3864,"record_offset":3448,"parent_record_offset":3416,"ancestor_record_offset":3064,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3864,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":3064,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3864,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3064,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3864,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":3064,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3864,"record_offset":4176,"parent_record_offset":4068,"ancestor_record_offset":3064,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3864,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":3064,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3864,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3064,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3864,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":3064,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3864,"record_offset":4012,"parent_record_offset":3980,"ancestor_record_offset":3064,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3864,"record_offset":3956,"parent_record_offset":3932,"ancestor_record_offset":3064,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3864,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":3064,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3864,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3064,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3864,"record_offset":4504,"parent_record_offset":3980,"ancestor_record_offset":3064,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3612,"parent_record_offset":3580,"ancestor_record_offset":3064,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3772,"parent_record_offset":3612,"ancestor_record_offset":3064,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3580,"parent_record_offset":3556,"ancestor_record_offset":3064,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3324,"parent_record_offset":3288,"ancestor_record_offset":3064,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3064,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3388,"parent_record_offset":3352,"ancestor_record_offset":3064,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3864,"record_offset":3352,"parent_record_offset":3324,"ancestor_record_offset":3064,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3864,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":3064,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3864,"record_offset":4316,"parent_record_offset":4068,"ancestor_record_offset":3064,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3864,"record_offset":4288,"parent_record_offset":4068,"ancestor_record_offset":3064,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3864,"record_offset":4120,"parent_record_offset":4068,"ancestor_record_offset":3064,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3864,"record_offset":8684,"parent_record_offset":8656,"ancestor_record_offset":3064,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3864,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":3064,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3864,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":3064,"inlinee":25252,"source_line":180,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3864,"record_offset":8824,"parent_record_offset":8800,"ancestor_record_offset":3064,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3864,"record_offset":9268,"parent_record_offset":8824,"ancestor_record_offset":3064,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3864,"record_offset":8896,"parent_record_offset":8868,"ancestor_record_offset":3064,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3864,"record_offset":8868,"parent_record_offset":8824,"ancestor_record_offset":3064,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3864,"record_offset":7076,"parent_record_offset":7052,"ancestor_record_offset":3064,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3864,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":3064,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":6548,"parent_record_offset":6240,"ancestor_record_offset":3064,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":7184,"parent_record_offset":7124,"ancestor_record_offset":3064,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":6240,"parent_record_offset":6212,"ancestor_record_offset":3064,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":3064,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":7156,"parent_record_offset":7124,"ancestor_record_offset":3064,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":3064,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3864,"record_offset":6320,"parent_record_offset":6288,"ancestor_record_offset":3064,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3864,"record_offset":6352,"parent_record_offset":6320,"ancestor_record_offset":3064,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3864,"record_offset":6288,"parent_record_offset":6240,"ancestor_record_offset":3064,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3864,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":3064,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3864,"record_offset":6416,"parent_record_offset":6240,"ancestor_record_offset":3064,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3864,"record_offset":6476,"parent_record_offset":6416,"ancestor_record_offset":3064,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":3064,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3864,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":3064,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":5200,"parent_record_offset":4996,"ancestor_record_offset":3064,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3864,"record_offset":5468,"parent_record_offset":5200,"ancestor_record_offset":3064,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3864,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":3064,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3864,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":3064,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3864,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":3064,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3864,"record_offset":5096,"parent_record_offset":5064,"ancestor_record_offset":3064,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3864,"record_offset":4996,"parent_record_offset":4960,"ancestor_record_offset":3064,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03864.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3864,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":3064,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03864.jsonl.gz"}
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
// 完整结构及原始类型记录: chimney_roof.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: chimney_roof.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: chimney_roof.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys
// 信息 ID: 6f7f210df95893bc227f8a8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d369c5182acd4f135dc6c1ad: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x17cb2f0 -> 0x1c64680 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x1c803a7 -> 0x1c64680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
