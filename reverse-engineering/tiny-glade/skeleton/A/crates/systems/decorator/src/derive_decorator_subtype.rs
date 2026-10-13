// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/derive_decorator_subtype.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: d8de609676bbe9dc5200eee480ee51feb9f9ca78ddb016ba7f4550badc829f5a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: derive_decorator_subtype.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2761, 2773, 3847
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
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
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtypes
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2773,"record_offset":4192,"rva":24994736,"va":5393703856}
// 编译实例 RVAs: 0x17d63b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24985262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e852c182fe","size":5,"soft_fail":false,"target_va":5368720672,"va":5393705929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405977c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19133817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8870adcfe","size":5,"soft_fail":false,"target_va":5374572480,"va":5393706292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18992495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e89132defe","size":5,"soft_fail":false,"target_va":5374712880,"va":5393705370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18988150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e88a43defe","size":5,"soft_fail":false,"target_va":5374716816,"va":5393704961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18496530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8eec3e5fe","size":5,"soft_fail":false,"target_va":5375209776,"va":5393706301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11450660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8dc4651ff","size":5,"soft_fail":false,"target_va":5382255760,"va":5393706415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ceb650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11449690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8a64a51ff","size":5,"soft_fail":false,"target_va":5382256208,"va":5393705893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfdb40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11374117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8db7152ff","size":5,"soft_fail":false,"target_va":5382331200,"va":5393705312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d06720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11339317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8cbf952ff","size":5,"soft_fail":false,"target_va":5382367008,"va":5393706320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c43660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4638645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8b5c74600","size":5,"soft_fail":false,"target_va":5398345312,"va":5393706662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9888712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8c8e39600","size":5,"soft_fail":false,"target_va":5403593392,"va":5393704675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9271469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8ad788d00","size":5,"soft_fail":false,"target_va":5402978688,"va":5393707214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9272187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e87b7b8d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5393706688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9272330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e80a7c8d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5393706817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9272151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8577b8d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5393706788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8acefffff","size":5,"soft_fail":false,"target_va":5393702336,"va":5393706511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9275699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e833898d00","size":5,"soft_fail":false,"target_va":5402980128,"va":5393704424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e82ffaffff","size":5,"soft_fail":false,"target_va":5393702528,"va":5393704012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9276562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8928c8d00","size":5,"soft_fail":false,"target_va":5402980656,"va":5393704089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19343965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8a3d5d8fe","size":5,"soft_fail":false,"target_va":5374362208,"va":5393706168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18986043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8c54bdefe","size":5,"soft_fail":false,"target_va":5374718704,"va":5393704742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11772938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8f65b4cff","size":5,"soft_fail":false,"target_va":5381932864,"va":5393705797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11772764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8a45c4cff","size":5,"soft_fail":false,"target_va":5381933056,"va":5393705815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18270806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8aa35e9fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5393705777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16353320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8d87706ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5393707091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17347683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e89d4bf7fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5393704590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17343797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8cb5af7fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5393704352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17347846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8fa4af7fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5393703985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11414569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8d7d351ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5393704372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::derive_decorator_subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e84f090000","size":5,"soft_fail":false,"target_va":5393707664,"va":5393705276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11449377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393703856,"raw_hex":"e8df4b51ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5393705964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2773,"record_offset":7068,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":13348,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2773,"record_offset":11196,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":13376,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2773,"record_offset":7516,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2773,"record_offset":9580,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2773,"record_offset":7488,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":13168,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":11160,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2773,"record_offset":11280,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2773,"record_offset":11224,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2773,"record_offset":13404,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2773,"record_offset":13264,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":6020,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":4432,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":5008,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":7128,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":9524,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":8524,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":8856,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::derive_decorator_subtype::can_become_full_chimney
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":11364,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":89929,"source_line":290,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2773,"record_offset":11336,"parent_record_offset":4192,"ancestor_record_offset":4192,"inlinee":60026,"source_line":231,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2773,"record_offset":7096,"parent_record_offset":7068,"ancestor_record_offset":4192,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6420,"parent_record_offset":6392,"ancestor_record_offset":4192,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":4192,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6448,"parent_record_offset":6420,"ancestor_record_offset":4192,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6360,"parent_record_offset":6332,"ancestor_record_offset":4192,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6392,"parent_record_offset":6360,"ancestor_record_offset":4192,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6564,"parent_record_offset":6392,"ancestor_record_offset":4192,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2773,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":4192,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2773,"record_offset":8328,"parent_record_offset":8304,"ancestor_record_offset":4192,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":7944,"parent_record_offset":7912,"ancestor_record_offset":4192,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":8008,"parent_record_offset":7972,"ancestor_record_offset":4192,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":4192,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2773,"record_offset":7396,"parent_record_offset":7360,"ancestor_record_offset":4192,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2773,"record_offset":7540,"parent_record_offset":7516,"ancestor_record_offset":4192,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":2773,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":4192,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2773,"record_offset":7632,"parent_record_offset":7604,"ancestor_record_offset":4192,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11736,"parent_record_offset":11364,"ancestor_record_offset":4192,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11808,"parent_record_offset":11768,"ancestor_record_offset":4192,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":4192,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2773,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":4192,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2773,"record_offset":4644,"parent_record_offset":4548,"ancestor_record_offset":4192,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":7872,"parent_record_offset":7848,"ancestor_record_offset":4192,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2773,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":4192,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2773,"record_offset":11304,"parent_record_offset":11280,"ancestor_record_offset":4192,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2773,"record_offset":11984,"parent_record_offset":11936,"ancestor_record_offset":4192,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":2773,"record_offset":13196,"parent_record_offset":13168,"ancestor_record_offset":4192,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2773,"record_offset":7604,"parent_record_offset":7540,"ancestor_record_offset":4192,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2773,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":4192,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":7576,"parent_record_offset":7540,"ancestor_record_offset":4192,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":6256,"parent_record_offset":6228,"ancestor_record_offset":4192,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":8376,"parent_record_offset":7540,"ancestor_record_offset":4192,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":4192,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":5604,"parent_record_offset":5144,"ancestor_record_offset":4192,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":6304,"parent_record_offset":6128,"ancestor_record_offset":4192,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":8436,"parent_record_offset":7516,"ancestor_record_offset":4192,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":4192,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2773,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":4192,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2773,"record_offset":6628,"parent_record_offset":6332,"ancestor_record_offset":4192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":4192,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":4192,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2773,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":4192,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2773,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":4192,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2773,"record_offset":5564,"parent_record_offset":5532,"ancestor_record_offset":4192,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2773,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":4192,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":7028,"parent_record_offset":6092,"ancestor_record_offset":4192,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":4580,"parent_record_offset":4548,"ancestor_record_offset":4192,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":4608,"parent_record_offset":4580,"ancestor_record_offset":4192,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2773,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":4192,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2773,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4192,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2773,"record_offset":5308,"parent_record_offset":5200,"ancestor_record_offset":4192,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2773,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":4192,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2773,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":4192,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2773,"record_offset":5172,"parent_record_offset":5144,"ancestor_record_offset":4192,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":2773,"record_offset":5144,"parent_record_offset":5112,"ancestor_record_offset":4192,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2773,"record_offset":5088,"parent_record_offset":5064,"ancestor_record_offset":4192,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2773,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":4192,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2773,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":4192,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":2773,"record_offset":5636,"parent_record_offset":5112,"ancestor_record_offset":4192,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4744,"parent_record_offset":4712,"ancestor_record_offset":4192,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4904,"parent_record_offset":4744,"ancestor_record_offset":4192,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":4192,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4464,"parent_record_offset":4432,"ancestor_record_offset":4192,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4928,"parent_record_offset":4904,"ancestor_record_offset":4192,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4520,"parent_record_offset":4488,"ancestor_record_offset":4192,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2773,"record_offset":4488,"parent_record_offset":4464,"ancestor_record_offset":4192,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2773,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":4192,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2773,"record_offset":5448,"parent_record_offset":5200,"ancestor_record_offset":4192,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2773,"record_offset":5420,"parent_record_offset":5200,"ancestor_record_offset":4192,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":2773,"record_offset":5252,"parent_record_offset":5200,"ancestor_record_offset":4192,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2773,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":4192,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2773,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":4192,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2773,"record_offset":12524,"parent_record_offset":11844,"ancestor_record_offset":4192,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":2773,"record_offset":8304,"parent_record_offset":8280,"ancestor_record_offset":4192,"inlinee":25252,"source_line":180,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12276,"parent_record_offset":12156,"ancestor_record_offset":4192,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12156,"parent_record_offset":12120,"ancestor_record_offset":4192,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12120,"parent_record_offset":11844,"ancestor_record_offset":4192,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12216,"parent_record_offset":12184,"ancestor_record_offset":4192,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11908,"parent_record_offset":11844,"ancestor_record_offset":4192,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":12044,"parent_record_offset":11936,"ancestor_record_offset":4192,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":4192,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":4192,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":4192,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":4192,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=9
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":8044,"parent_record_offset":7736,"ancestor_record_offset":4192,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":8656,"parent_record_offset":8596,"ancestor_record_offset":4192,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":4192,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":4192,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":4192,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":8572,"parent_record_offset":8548,"ancestor_record_offset":4192,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":4192,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7360,"parent_record_offset":7300,"ancestor_record_offset":4192,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":8628,"parent_record_offset":8596,"ancestor_record_offset":4192,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7332,"parent_record_offset":7300,"ancestor_record_offset":4192,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":4192,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":7816,"parent_record_offset":7784,"ancestor_record_offset":4192,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":7848,"parent_record_offset":7816,"ancestor_record_offset":4192,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":10784,"parent_record_offset":10656,"ancestor_record_offset":4192,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":7784,"parent_record_offset":7736,"ancestor_record_offset":4192,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7428,"parent_record_offset":7300,"ancestor_record_offset":4192,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":4192,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":4192,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":7912,"parent_record_offset":7736,"ancestor_record_offset":4192,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":7972,"parent_record_offset":7912,"ancestor_record_offset":4192,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\scopeguard.rs
//     evidence={"module":2773,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":4192,"inlinee":8805,"source_line":69,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":6056,"parent_record_offset":6020,"ancestor_record_offset":4192,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2773,"record_offset":6168,"parent_record_offset":6128,"ancestor_record_offset":4192,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":6332,"parent_record_offset":6128,"ancestor_record_offset":4192,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2773,"record_offset":6600,"parent_record_offset":6332,"ancestor_record_offset":4192,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":11404,"parent_record_offset":11364,"ancestor_record_offset":4192,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":7660,"parent_record_offset":7632,"ancestor_record_offset":4192,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":4192,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12080,"parent_record_offset":12044,"ancestor_record_offset":4192,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2773,"record_offset":6196,"parent_record_offset":6168,"ancestor_record_offset":4192,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2773,"record_offset":6228,"parent_record_offset":6196,"ancestor_record_offset":4192,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2773,"record_offset":6128,"parent_record_offset":6092,"ancestor_record_offset":4192,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2773,"record_offset":6092,"parent_record_offset":6056,"ancestor_record_offset":4192,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":11844,"parent_record_offset":11808,"ancestor_record_offset":4192,"inlinee":89931,"source_line":310,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":4192,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":4192,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12328,"parent_record_offset":12276,"ancestor_record_offset":4192,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":4192,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2773,"record_offset":12368,"parent_record_offset":11844,"ancestor_record_offset":4192,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F23
// 首条位置: {"module":2761,"inlinee":89891}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2761,"record_offset":19436,"parent_record_offset":19412,"ancestor_record_offset":19412,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2761,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":19412,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2761,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":19412,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2761,"record_offset":19768,"parent_record_offset":19744,"ancestor_record_offset":19412,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2761,"record_offset":19708,"parent_record_offset":19684,"ancestor_record_offset":19412,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2761,"record_offset":19880,"parent_record_offset":19856,"ancestor_record_offset":19412,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2761,"record_offset":20128,"parent_record_offset":20104,"ancestor_record_offset":19412,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2761,"record_offset":20020,"parent_record_offset":19996,"ancestor_record_offset":19412,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2761,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":19412,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2761,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":19412,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2761,"record_offset":19460,"parent_record_offset":19436,"ancestor_record_offset":19412,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2761,"record_offset":19484,"parent_record_offset":19460,"ancestor_record_offset":19412,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2761,"record_offset":20168,"parent_record_offset":19948,"ancestor_record_offset":19412,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2761,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":19412,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":20080,"parent_record_offset":20056,"ancestor_record_offset":19412,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":20056,"parent_record_offset":19948,"ancestor_record_offset":19412,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19996,"parent_record_offset":19972,"ancestor_record_offset":19412,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":20376,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19972,"parent_record_offset":19948,"ancestor_record_offset":19412,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19564,"parent_record_offset":19540,"ancestor_record_offset":19412,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19540,"parent_record_offset":19484,"ancestor_record_offset":19412,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":19412,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2761,"record_offset":19828,"parent_record_offset":19800,"ancestor_record_offset":19412,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2761,"record_offset":19856,"parent_record_offset":19828,"ancestor_record_offset":19412,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2761,"record_offset":19916,"parent_record_offset":19800,"ancestor_record_offset":19412,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2761,"record_offset":19800,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19632,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2761,"record_offset":19948,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2761,"record_offset":19660,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2761,"record_offset":19744,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2761,"record_offset":20284,"parent_record_offset":19588,"ancestor_record_offset":19412,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype
// 记录形式: compiled_procedure; instances=1
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2773,"record_offset":13464,"rva":24998544,"va":5393707664}
// 编译实例 RVAs: 0x17d7290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9269630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e87e718d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5393709245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9269938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e8b2728d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5393709209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9269810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e832728d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5393709129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17944425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e89730eefe","size":5,"soft_fail":false,"target_va":5375763552,"va":5393707972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::WallId::is_valid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18887631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e831ccdffe","size":5,"soft_fail":false,"target_va":5374821312,"va":5393708938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18991085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e81338defe","size":5,"soft_fail":false,"target_va":5374716704,"va":5393707784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18273110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e8aa2ce9fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5393708081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-18271768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e8e831e9fe","size":5,"soft_fail":false,"target_va":5375436032,"va":5393707795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-18272276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e8ec2fe9fe","size":5,"soft_fail":false,"target_va":5375436576,"va":5393708847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::is_half_dormer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393707664,"raw_hex":"e88f010000","size":5,"soft_fail":false,"target_va":5393709328,"va":5393708924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":17852,"parent_record_offset":13464,"ancestor_record_offset":13464,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":13628,"parent_record_offset":13464,"ancestor_record_offset":13464,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14640,"parent_record_offset":13464,"ancestor_record_offset":13464,"inlinee":89940,"source_line":628,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14696,"parent_record_offset":13464,"ancestor_record_offset":13464,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2773,"record_offset":13896,"parent_record_offset":13752,"ancestor_record_offset":13464,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2773,"record_offset":13752,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14540,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14172,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":13464,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2773,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":13464,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2773,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":13464,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":13464,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":13464,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":13464,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":13464,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14476,"parent_record_offset":14196,"ancestor_record_offset":13464,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14224,"parent_record_offset":14196,"ancestor_record_offset":13464,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14144,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2773,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":13464,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2773,"record_offset":14564,"parent_record_offset":14540,"ancestor_record_offset":13464,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2773,"record_offset":13960,"parent_record_offset":13928,"ancestor_record_offset":13464,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13464,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":13840,"parent_record_offset":13752,"ancestor_record_offset":13464,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":13464,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":13464,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":14068,"parent_record_offset":14036,"ancestor_record_offset":13464,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":14116,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":13784,"parent_record_offset":13752,"ancestor_record_offset":13464,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":13464,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":13928,"parent_record_offset":13708,"ancestor_record_offset":13464,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":14012,"parent_record_offset":13988,"ancestor_record_offset":13464,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":13464,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":13464,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2773,"record_offset":14432,"parent_record_offset":14408,"ancestor_record_offset":13464,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":15772,"parent_record_offset":15744,"ancestor_record_offset":13464,"inlinee":89943,"source_line":129,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":16772,"parent_record_offset":16748,"ancestor_record_offset":13464,"inlinee":89944,"source_line":130,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":14760,"parent_record_offset":14732,"ancestor_record_offset":13464,"inlinee":89942,"source_line":149,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":13464,"inlinee":89939,"source_line":174,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":13464,"inlinee":89941,"source_line":175,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":16868,"parent_record_offset":16840,"ancestor_record_offset":13464,"inlinee":89945,"source_line":200,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F57
// 首条位置: {"module":2773,"inlinee":89943}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2773,"record_offset":15796,"parent_record_offset":15772,"ancestor_record_offset":15772,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2773,"record_offset":16012,"parent_record_offset":15868,"ancestor_record_offset":15772,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2773,"record_offset":15868,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":16648,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":16280,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":15772,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2773,"record_offset":15824,"parent_record_offset":15796,"ancestor_record_offset":15772,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2773,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":15772,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":15772,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":15772,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":15772,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":15772,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":16584,"parent_record_offset":16304,"ancestor_record_offset":15772,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":16332,"parent_record_offset":16304,"ancestor_record_offset":15772,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":16252,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2773,"record_offset":16672,"parent_record_offset":16648,"ancestor_record_offset":15772,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2773,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":15772,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":15772,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":15956,"parent_record_offset":15868,"ancestor_record_offset":15772,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":15772,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":15772,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":16176,"parent_record_offset":16144,"ancestor_record_offset":15772,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":16224,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":15900,"parent_record_offset":15868,"ancestor_record_offset":15772,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":15772,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":16044,"parent_record_offset":15824,"ancestor_record_offset":15772,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":15772,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":16144,"parent_record_offset":16120,"ancestor_record_offset":15772,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":15772,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2773,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":15772,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F58
// 首条位置: {"module":2773,"inlinee":89944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F56
// 首条位置: {"module":2773,"inlinee":89942}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2773,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":14760,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2773,"record_offset":15000,"parent_record_offset":14856,"ancestor_record_offset":14760,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2773,"record_offset":14856,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":15644,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":15276,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":14760,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2773,"record_offset":14812,"parent_record_offset":14784,"ancestor_record_offset":14760,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2773,"record_offset":15604,"parent_record_offset":15580,"ancestor_record_offset":14760,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":15376,"parent_record_offset":15352,"ancestor_record_offset":14760,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":15352,"parent_record_offset":15328,"ancestor_record_offset":14760,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":14760,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":15512,"parent_record_offset":15488,"ancestor_record_offset":14760,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":15580,"parent_record_offset":15300,"ancestor_record_offset":14760,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":15328,"parent_record_offset":15300,"ancestor_record_offset":14760,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":15248,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2773,"record_offset":15668,"parent_record_offset":15644,"ancestor_record_offset":14760,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2773,"record_offset":15064,"parent_record_offset":15032,"ancestor_record_offset":14760,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":14968,"parent_record_offset":14944,"ancestor_record_offset":14760,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":14944,"parent_record_offset":14856,"ancestor_record_offset":14760,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":14760,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":14760,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":14760,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":15220,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":14888,"parent_record_offset":14856,"ancestor_record_offset":14760,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":14760,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":15032,"parent_record_offset":14812,"ancestor_record_offset":14760,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":14760,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":14760,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":15092,"parent_record_offset":15064,"ancestor_record_offset":14760,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2773,"record_offset":15536,"parent_record_offset":15512,"ancestor_record_offset":14760,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F53
// 首条位置: {"module":2773,"inlinee":89939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2773,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":13656,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2773,"record_offset":13896,"parent_record_offset":13752,"ancestor_record_offset":13656,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2773,"record_offset":13752,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14540,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14172,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":13656,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2773,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":13656,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2773,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":13656,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":13656,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":13656,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":13656,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":13656,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14476,"parent_record_offset":14196,"ancestor_record_offset":13656,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14224,"parent_record_offset":14196,"ancestor_record_offset":13656,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":14144,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2773,"record_offset":14564,"parent_record_offset":14540,"ancestor_record_offset":13656,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2773,"record_offset":13960,"parent_record_offset":13928,"ancestor_record_offset":13656,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13656,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":13840,"parent_record_offset":13752,"ancestor_record_offset":13656,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":13656,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":13656,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":14068,"parent_record_offset":14036,"ancestor_record_offset":13656,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":14116,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":13784,"parent_record_offset":13752,"ancestor_record_offset":13656,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":13656,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":13928,"parent_record_offset":13708,"ancestor_record_offset":13656,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":14012,"parent_record_offset":13988,"ancestor_record_offset":13656,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":13656,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":13656,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2773,"record_offset":14432,"parent_record_offset":14408,"ancestor_record_offset":13656,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F55
// 首条位置: {"module":2773,"inlinee":89941}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F59
// 首条位置: {"module":2773,"inlinee":89945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":2773,"record_offset":16892,"parent_record_offset":16868,"ancestor_record_offset":16868,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":2773,"record_offset":17108,"parent_record_offset":16964,"ancestor_record_offset":16868,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":2773,"record_offset":16964,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":17752,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":17384,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":2773,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":16868,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":2773,"record_offset":16920,"parent_record_offset":16892,"ancestor_record_offset":16868,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":2773,"record_offset":17712,"parent_record_offset":17688,"ancestor_record_offset":16868,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":16868,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":17460,"parent_record_offset":17436,"ancestor_record_offset":16868,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":16868,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":2773,"record_offset":17620,"parent_record_offset":17596,"ancestor_record_offset":16868,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":17688,"parent_record_offset":17408,"ancestor_record_offset":16868,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":17436,"parent_record_offset":17408,"ancestor_record_offset":16868,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":2773,"record_offset":17356,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":2773,"record_offset":17776,"parent_record_offset":17752,"ancestor_record_offset":16868,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":2773,"record_offset":17172,"parent_record_offset":17140,"ancestor_record_offset":16868,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":17076,"parent_record_offset":17052,"ancestor_record_offset":16868,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":17052,"parent_record_offset":16964,"ancestor_record_offset":16868,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":17556,"parent_record_offset":17532,"ancestor_record_offset":16868,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":17508,"parent_record_offset":17484,"ancestor_record_offset":16868,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":17280,"parent_record_offset":17248,"ancestor_record_offset":16868,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2773,"record_offset":17328,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":16996,"parent_record_offset":16964,"ancestor_record_offset":16868,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":17020,"parent_record_offset":16996,"ancestor_record_offset":16868,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":17140,"parent_record_offset":16920,"ancestor_record_offset":16868,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":16868,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":17248,"parent_record_offset":17224,"ancestor_record_offset":16868,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":2773,"record_offset":17200,"parent_record_offset":17172,"ancestor_record_offset":16868,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":2773,"record_offset":17644,"parent_record_offset":17620,"ancestor_record_offset":16868,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::is_half_dormer
// 记录形式: compiled_procedure; instances=1
// 源码行: 240
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2773,"record_offset":17968,"rva":25000208,"va":5393709328}
// 编译实例 RVAs: 0x17d7910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140633130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18499921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8afb6e5fe","size":5,"soft_fail":false,"target_va":5375209776,"va":5393709692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16766115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e85d2b00ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5393710110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11799842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8def24bff","size":5,"soft_fail":false,"target_va":5381909888,"va":5393709725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf3540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: derive_decorator_subtype.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11420901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e81bbb51ff","size":5,"soft_fail":false,"target_va":5382288704,"va":5393709600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9268531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8336d8d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5393710344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9268815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e84f6e8d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5393710332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9269134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e88e6f8d00","size":5,"soft_fail":false,"target_va":5402979344,"va":5393710205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9268631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8976d8d00","size":5,"soft_fail":false,"target_va":5402978944,"va":5393710308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17945857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8ff2aeefe","size":5,"soft_fail":false,"target_va":5375763616,"va":5393709468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17945873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8ef2aeefe","size":5,"soft_fail":false,"target_va":5375763552,"va":5393709420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17945876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8ec2aeefe","size":5,"soft_fail":false,"target_va":5375763568,"va":5393709439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17984465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e82f94edfe","size":5,"soft_fail":false,"target_va":5375724992,"va":5393709452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15748396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8d4b20fff","size":5,"soft_fail":false,"target_va":5377961136,"va":5393709527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11452920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393709328,"raw_hex":"e8083e51ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5393709507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":18164,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2773,"record_offset":18200,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":18564,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":21727,"source_line":199,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":2773,"record_offset":18136,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":2773,"record_offset":18340,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::center
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":19964,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":22795,"source_line":60,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":19992,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":18296,"parent_record_offset":18272,"ancestor_record_offset":17968,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2773,"record_offset":19508,"parent_record_offset":19480,"ancestor_record_offset":17968,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":18880,"parent_record_offset":18852,"ancestor_record_offset":17968,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":18976,"parent_record_offset":18948,"ancestor_record_offset":17968,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":19924,"parent_record_offset":19900,"ancestor_record_offset":17968,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2773,"record_offset":18908,"parent_record_offset":18880,"ancestor_record_offset":17968,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":19124,"parent_record_offset":19096,"ancestor_record_offset":17968,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":19876,"parent_record_offset":18564,"ancestor_record_offset":17968,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":19408,"parent_record_offset":19380,"ancestor_record_offset":17968,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":19284,"parent_record_offset":19256,"ancestor_record_offset":17968,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2773,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":17968,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2773,"record_offset":18272,"parent_record_offset":18248,"ancestor_record_offset":17968,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":18404,"parent_record_offset":18340,"ancestor_record_offset":17968,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2773,"record_offset":18432,"parent_record_offset":18404,"ancestor_record_offset":17968,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2773,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":17968,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":19536,"parent_record_offset":19508,"ancestor_record_offset":17968,"inlinee":26328,"source_line":78,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":17968,"inlinee":21755,"source_line":202,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":2773,"record_offset":19480,"parent_record_offset":19452,"ancestor_record_offset":17968,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":17968,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":17968,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":19452,"parent_record_offset":19200,"ancestor_record_offset":17968,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":18672,"parent_record_offset":18644,"ancestor_record_offset":17968,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19352,"parent_record_offset":19324,"ancestor_record_offset":17968,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19324,"parent_record_offset":19200,"ancestor_record_offset":17968,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19256,"parent_record_offset":19228,"ancestor_record_offset":17968,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19700,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19228,"parent_record_offset":19200,"ancestor_record_offset":17968,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":18724,"parent_record_offset":18700,"ancestor_record_offset":17968,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":18700,"parent_record_offset":18644,"ancestor_record_offset":17968,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":18752,"parent_record_offset":18724,"ancestor_record_offset":17968,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":19052,"parent_record_offset":19012,"ancestor_record_offset":17968,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":19096,"parent_record_offset":19052,"ancestor_record_offset":17968,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":19168,"parent_record_offset":19012,"ancestor_record_offset":17968,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":19012,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":18824,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":19200,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":18852,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":18948,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":19584,"parent_record_offset":18752,"ancestor_record_offset":17968,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":18596,"parent_record_offset":18564,"ancestor_record_offset":17968,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":2773,"record_offset":18376,"parent_record_offset":18340,"ancestor_record_offset":17968,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":2773,"record_offset":18468,"parent_record_offset":18340,"ancestor_record_offset":17968,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::can_become_full_chimney
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F49
// 首条位置: {"module":2773,"inlinee":89929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11736,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":11404,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":11592,"parent_record_offset":11564,"ancestor_record_offset":11364,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2773,"record_offset":11652,"parent_record_offset":11620,"ancestor_record_offset":11364,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":11364,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11808,"parent_record_offset":11768,"ancestor_record_offset":11364,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2773,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":11364,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2773,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":11364,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2773,"record_offset":11984,"parent_record_offset":11936,"ancestor_record_offset":11364,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":11364,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2773,"record_offset":11688,"parent_record_offset":11536,"ancestor_record_offset":11364,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":11364,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2773,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":11364,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2773,"record_offset":12524,"parent_record_offset":11844,"ancestor_record_offset":11364,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12276,"parent_record_offset":12156,"ancestor_record_offset":11364,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12156,"parent_record_offset":12120,"ancestor_record_offset":11364,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12120,"parent_record_offset":11844,"ancestor_record_offset":11364,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12216,"parent_record_offset":12184,"ancestor_record_offset":11364,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11908,"parent_record_offset":11844,"ancestor_record_offset":11364,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":12044,"parent_record_offset":11936,"ancestor_record_offset":11364,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11364,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":11364,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2773,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":11364,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":12944,"parent_record_offset":12636,"ancestor_record_offset":11364,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":11536,"parent_record_offset":11476,"ancestor_record_offset":11364,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":12636,"parent_record_offset":12608,"ancestor_record_offset":11364,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":11452,"parent_record_offset":11428,"ancestor_record_offset":11364,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":11508,"parent_record_offset":11476,"ancestor_record_offset":11364,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":11364,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":12716,"parent_record_offset":12684,"ancestor_record_offset":11364,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":12748,"parent_record_offset":12716,"ancestor_record_offset":11364,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2773,"record_offset":12684,"parent_record_offset":12636,"ancestor_record_offset":11364,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2773,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":11364,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":11564,"parent_record_offset":11536,"ancestor_record_offset":11364,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2773,"record_offset":11620,"parent_record_offset":11564,"ancestor_record_offset":11364,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2773,"record_offset":12560,"parent_record_offset":11768,"ancestor_record_offset":11364,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":11364,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12080,"parent_record_offset":12044,"ancestor_record_offset":11364,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/derive_decorator_subtype.rs
//     evidence={"module":2773,"record_offset":11844,"parent_record_offset":11808,"ancestor_record_offset":11364,"inlinee":89931,"source_line":310,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":11364,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":11364,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12328,"parent_record_offset":12276,"ancestor_record_offset":11364,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":11364,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2773,"record_offset":12368,"parent_record_offset":11844,"ancestor_record_offset":11364,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F4B
// 首条位置: {"module":2773,"inlinee":89931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2773,"record_offset":12524,"parent_record_offset":11844,"ancestor_record_offset":11844,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12120,"parent_record_offset":11844,"ancestor_record_offset":11844,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11908,"parent_record_offset":11844,"ancestor_record_offset":11844,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2773,"record_offset":12368,"parent_record_offset":11844,"ancestor_record_offset":11844,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2773,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":11844,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2773,"record_offset":11984,"parent_record_offset":11936,"ancestor_record_offset":11844,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2773,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":11844,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12276,"parent_record_offset":12156,"ancestor_record_offset":11844,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12156,"parent_record_offset":12120,"ancestor_record_offset":11844,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2773,"record_offset":12216,"parent_record_offset":12184,"ancestor_record_offset":11844,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":12044,"parent_record_offset":11936,"ancestor_record_offset":11844,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2773,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11844,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":11844,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2773,"record_offset":12080,"parent_record_offset":12044,"ancestor_record_offset":11844,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":11844,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2773,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":11844,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12328,"parent_record_offset":12276,"ancestor_record_offset":11844,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02773.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2773,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":11844,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/02773.jsonl.gz"}
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
// 完整结构及原始类型记录: derive_decorator_subtype.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: derive_decorator_subtype.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: derive_decorator_subtype.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::can_become_full_chimney
// 信息 ID: d2aac0db65dffc85d10a9ca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=2773, inline_record=11364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::can_become_full_chimney::closure$0
// 信息 ID: 7b062e10f7d2e4d1ec48e398
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::can_become_full_chimney", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtypes; module=2773, inline_record=11844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype
// 信息 ID: 79ca34a78f8abec85806ddf5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0aa69 -> 0x17d7290 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a2f1 -> 0x17d7290 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes @ 0x17d693c -> 0x17d7290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$0
// 信息 ID: 52cca615f49bc1a1b3b6fcc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=15772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$1
// 信息 ID: 1a72dff3e52431079bc881cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=16772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$2
// 信息 ID: 2b830585f204a27b3d6d3a1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=14760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$3
// 信息 ID: 5df70ca2cde2b11ea70549ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=13656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$4
// 信息 ID: 960cffa4452d9315bcc72da2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=14664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtype::closure$5
// 信息 ID: 11a1b6fd954f8677af020f54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtype", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::derive_decorator_subtype::derive_decorator_subtype; module=2773, inline_record=16868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtypes
// 信息 ID: 648d52c78020b7cd78c31a10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 943e756bb07dafeded25746f: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> > @ 0x17c9df6 -> 0x17d63b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)> @ 0x17ec10e -> 0x17d63b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure$0
// 信息 ID: 7a86e826d0b91343f9f0d331
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::derive_decorator_subtype::derive_decorator_subtypes", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$0,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0 <- core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$1>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$0,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0> >,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; module=2761, inline_record=19412; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::chain::impl$1::try_fold<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl$4::iter::closure_env$1>,tuple$<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env$0<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator_storage::Decorators> >,tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env$0>,enum2$<core::ops::control_flow::ControlFlow<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<> > > >; module=3847, inline_record=1832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::derive_decorator_subtype::is_half_dormer
// 信息 ID: 7b857b27348d9dc37adb8f17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::derive_decorator_subtype::derive_decorator_subtype @ 0x17d77df -> 0x17d7910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
