// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/clutter/src/clutter/add_clutter_around_windows.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4bff2c4b17ef6e80daebd00cd7b4399d17d28fa6427fd719031821107897f3c1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: add_clutter_around_windows.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3509
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
// use system_clutter; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3509,"record_offset":3060,"rva":28109920,"va":5396819040}
// 编译实例 RVAs: 0x1acec60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=6
//     target_aliases=53 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23670869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8abcf96fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5396820704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140597150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22249415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e83980acfe","size":5,"soft_fail":false,"target_va":5374570832,"va":5396820242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9c30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22106973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8a3acaefe","size":5,"soft_fail":false,"target_va":5374712880,"va":5396819848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21606598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e83a4fb6fe","size":5,"soft_fail":false,"target_va":5375213232,"va":5396819825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19876951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8a9b3d0fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5396820914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19876830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e822b4d0fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5396820825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ade900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t62521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e839f40000","size":5,"soft_fail":false,"target_va":5396883712,"va":5396821186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141adf000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t64729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8d9fc0000","size":5,"soft_fail":false,"target_va":5396885504,"va":5396820770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141adf6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: add_clutter_around_windows.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8e0000100","size":5,"soft_fail":false,"target_va":5396887248,"va":5396821483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6156249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8d9ef5d00","size":5,"soft_fail":false,"target_va":5402978688,"va":5396822434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6156455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8a7f05d00","size":5,"soft_fail":false,"target_va":5402978880,"va":5396822420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6156767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8dff15d00","size":5,"soft_fail":false,"target_va":5402979152,"va":5396822380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6157019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8dbf25d00","size":5,"soft_fail":false,"target_va":5402979344,"va":5396822320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e899eeffff","size":5,"soft_fail":false,"target_va":5396817760,"va":5396822210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6160512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e880005e00","size":5,"soft_fail":false,"target_va":5402980128,"va":5396819611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e828fbffff","size":5,"soft_fail":false,"target_va":5396817952,"va":5396819187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6161387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8eb035e00","size":5,"soft_fail":false,"target_va":5402980656,"va":5396819264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22457817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e82752a9fe","size":5,"soft_fail":false,"target_va":5374362576,"va":5396820388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21607662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8124bb6fe","size":5,"soft_fail":false,"target_va":5375212688,"va":5396820345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21385112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e868b0b9fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5396820083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20462873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e8e7c2c7fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5396819780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20458984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e818d2c7fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5396819539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20463029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e84bc2c7fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5396819168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h93c6532eedf5b01e (.llvm.12250732642055207085)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17896924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e824eaeefe","size":5,"soft_fail":false,"target_va":5378922640,"va":5396819559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_clutter::clutter::ClutterState::add_region
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17898093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396819040,"raw_hex":"e893e5eefe","size":5,"soft_fail":false,"target_va":5378922960,"va":5396821048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3509,"record_offset":9140,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3509,"record_offset":8308,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3509,"record_offset":6028,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3509,"record_offset":6000,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3509,"record_offset":5944,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3509,"record_offset":6620,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3509,"record_offset":7692,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":9948,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":106794,"source_line":500,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$11::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":8588,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":27855,"source_line":991,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":8700,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":8532,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3509,"record_offset":7604,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":4896,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":3308,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":3884,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3509,"record_offset":6564,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3509,"record_offset":6648,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3509,"record_offset":7008,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] system_clutter::clutter::ClutterRegion::centered
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3509,"record_offset":8756,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":48767,"source_line":438,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3509,"record_offset":9196,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":59684,"source_line":954,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3509,"record_offset":9484,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_unnormalized_central_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3509,"record_offset":9736,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":81210,"source_line":643,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3509,"record_offset":9512,"parent_record_offset":9484,"ancestor_record_offset":3060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3509,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3509,"record_offset":8412,"parent_record_offset":8336,"ancestor_record_offset":3060,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5296,"parent_record_offset":5268,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5236,"parent_record_offset":5208,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5268,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5440,"parent_record_offset":5268,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3509,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3509,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":3060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3509,"record_offset":6268,"parent_record_offset":6236,"ancestor_record_offset":3060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3509,"record_offset":6332,"parent_record_offset":6296,"ancestor_record_offset":3060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3509,"record_offset":8132,"parent_record_offset":7716,"ancestor_record_offset":3060,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3509,"record_offset":8040,"parent_record_offset":7984,"ancestor_record_offset":3060,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3509,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":3060,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3509,"record_offset":7844,"parent_record_offset":7760,"ancestor_record_offset":3060,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3509,"record_offset":7904,"parent_record_offset":7716,"ancestor_record_offset":3060,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3509,"record_offset":9340,"parent_record_offset":9196,"ancestor_record_offset":3060,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3509,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3509,"record_offset":9764,"parent_record_offset":9736,"ancestor_record_offset":3060,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3509,"record_offset":3520,"parent_record_offset":3424,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3509,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3509,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3509,"record_offset":7632,"parent_record_offset":7604,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3509,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3509,"record_offset":6528,"parent_record_offset":6496,"ancestor_record_offset":3060,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3509,"record_offset":8384,"parent_record_offset":8336,"ancestor_record_offset":3060,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3509,"record_offset":8336,"parent_record_offset":8308,"ancestor_record_offset":3060,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3509,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3509,"record_offset":4480,"parent_record_offset":4020,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3509,"record_offset":5180,"parent_record_offset":5004,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3509,"record_offset":9304,"parent_record_offset":9272,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3509,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3509,"record_offset":5504,"parent_record_offset":5208,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3509,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":3060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3509,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":3060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3509,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3509,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3509,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3509,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3509,"record_offset":5904,"parent_record_offset":4968,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3509,"record_offset":3456,"parent_record_offset":3424,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3509,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":3060,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3509,"record_offset":9540,"parent_record_offset":9512,"ancestor_record_offset":3060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3509,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3509,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":3060,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3509,"record_offset":9568,"parent_record_offset":9540,"ancestor_record_offset":3060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3509,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3509,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3509,"record_offset":4184,"parent_record_offset":4076,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3509,"record_offset":3424,"parent_record_offset":3396,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3509,"record_offset":3916,"parent_record_offset":3884,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3509,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3509,"record_offset":4020,"parent_record_offset":3988,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3509,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3509,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3509,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3509,"record_offset":4512,"parent_record_offset":3988,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3620,"parent_record_offset":3588,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3780,"parent_record_offset":3620,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3340,"parent_record_offset":3308,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3396,"parent_record_offset":3364,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3509,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3509,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3509,"record_offset":4324,"parent_record_offset":4076,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3509,"record_offset":4296,"parent_record_offset":4076,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3509,"record_offset":4128,"parent_record_offset":4076,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3509,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":3060,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3509,"record_offset":8160,"parent_record_offset":7716,"ancestor_record_offset":3060,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":10056,"parent_record_offset":10024,"ancestor_record_offset":3060,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":3060,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":10000,"parent_record_offset":9972,"ancestor_record_offset":3060,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":3060,"inlinee":106795,"source_line":483,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":8872,"parent_record_offset":8756,"ancestor_record_offset":3060,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3509,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":3060,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3509,"record_offset":9048,"parent_record_offset":8756,"ancestor_record_offset":3060,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3509,"record_offset":8960,"parent_record_offset":8756,"ancestor_record_offset":3060,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3509,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":3060,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3509,"record_offset":7760,"parent_record_offset":7716,"ancestor_record_offset":3060,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3509,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":3060,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3509,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":3060,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3509,"record_offset":6080,"parent_record_offset":6056,"ancestor_record_offset":3060,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6368,"parent_record_offset":6132,"ancestor_record_offset":3060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6804,"parent_record_offset":6744,"ancestor_record_offset":3060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":3060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":3060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":3060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":3060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3509,"record_offset":6200,"parent_record_offset":6172,"ancestor_record_offset":3060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3509,"record_offset":7196,"parent_record_offset":7164,"ancestor_record_offset":3060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3509,"record_offset":6172,"parent_record_offset":6132,"ancestor_record_offset":3060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3509,"record_offset":6104,"parent_record_offset":6080,"ancestor_record_offset":3060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3509,"record_offset":6236,"parent_record_offset":6132,"ancestor_record_offset":3060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3509,"record_offset":6296,"parent_record_offset":6236,"ancestor_record_offset":3060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":4932,"parent_record_offset":4896,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3509,"record_offset":5044,"parent_record_offset":5004,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":5208,"parent_record_offset":5004,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3509,"record_offset":5476,"parent_record_offset":5208,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3509,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":3060,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3509,"record_offset":6056,"parent_record_offset":6028,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3509,"record_offset":10088,"parent_record_offset":10056,"ancestor_record_offset":3060,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3509,"record_offset":5072,"parent_record_offset":5044,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3509,"record_offset":5104,"parent_record_offset":5072,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3509,"record_offset":5004,"parent_record_offset":4968,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3509,"record_offset":4968,"parent_record_offset":4932,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3509,"record_offset":9608,"parent_record_offset":9484,"ancestor_record_offset":3060,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9452,"parent_record_offset":9196,"ancestor_record_offset":3060,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9424,"parent_record_offset":9196,"ancestor_record_offset":3060,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9240,"parent_record_offset":9196,"ancestor_record_offset":3060,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":3060,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9392,"parent_record_offset":9368,"ancestor_record_offset":3060,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/03509.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":3509,"record_offset":9368,"parent_record_offset":9196,"ancestor_record_offset":3060,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/03509.jsonl.gz"}
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
// 完整结构及原始类型记录: add_clutter_around_windows.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: add_clutter_around_windows.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: add_clutter_around_windows.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows
// 信息 ID: 41de1a02a7f256ff0f33ed8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6d47b9c28fc8e3460f367794: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)> > @ 0x14c2419 -> 0x1acec60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_clutter::clutter::ClutterState>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources)> @ 0x1ae14e2 -> 0x1acec60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
