// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_visual/flag_connections.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 622be76caca34648de714d522776f4331f25bd5f8cc409966a2217611ab4cab6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flag_connections.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3838, 3870
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::generate_flag_connections
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3870,"record_offset":3060,"rva":29818656,"va":5398527776}
// 编译实例 RVAs: 0x1c6ff20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28916470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e80ac546fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5398529985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25782193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e84f9876fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398531900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=3
//     target_aliases=53 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25380714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e896b87cfe","size":5,"soft_fail":false,"target_va":5373149840,"va":5398530549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23812860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e804a594fe","size":5,"soft_fail":false,"target_va":5374716816,"va":5398529671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140633eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23316011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8d5399cfe","size":5,"soft_fail":false,"target_va":5375213232,"va":5398529238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5152069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8bb62b1ff","size":5,"soft_fail":false,"target_va":5393378592,"va":5398530656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4444535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e877d14300","size":5,"soft_fail":false,"target_va":5402978688,"va":5398534148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4445225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e829d44300","size":5,"soft_fail":false,"target_va":5402979344,"va":5398534114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-7411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e80de3ffff","size":5,"soft_fail":false,"target_va":5398526496,"va":5398533902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4451762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8b2ed4300","size":5,"soft_fail":false,"target_va":5402980128,"va":5398528361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-1252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e81cfbffff","size":5,"soft_fail":false,"target_va":5398526688,"va":5398527935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t4452639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e81ff14300","size":5,"soft_fail":false,"target_va":5402980656,"va":5398528012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24167636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e82c3b8ffe","size":5,"soft_fail":false,"target_va":5374362576,"va":5398530207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23810635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8b5ad94fe","size":5,"soft_fail":false,"target_va":5374718704,"va":5398529334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23814440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8d89e94fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5398531139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-23317543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8d9339cfe","size":5,"soft_fail":false,"target_va":5375212688,"va":5398530226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22171621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e81bb0adfe","size":5,"soft_fail":false,"target_va":5376356912,"va":5398528528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-22167734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e84abfadfe","size":5,"soft_fail":false,"target_va":5376360560,"va":5398528289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22171777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e87fafadfe","size":5,"soft_fail":false,"target_va":5376356144,"va":5398527916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::hcc99216871ec738f (.llvm.16000913382889057913)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16238506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e8563808ff","size":5,"soft_fail":false,"target_va":5382289808,"va":5398528309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::flag_connections::clothline
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e820150000","size":5,"soft_fail":false,"target_va":5398538928,"va":5398533515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::flag_connections::greedily_connect
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398527776,"raw_hex":"e84b120000","size":5,"soft_fail":false,"target_va":5398536512,"va":5398531824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":8668,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":13004,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":15944,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3870,"record_offset":15724,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3870,"record_offset":15184,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3870,"record_offset":15212,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3870,"record_offset":8752,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3870,"record_offset":8724,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":15504,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":15780,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":15864,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":3870,"record_offset":8612,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21526,"source_line":249,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":16232,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":11780,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":89885,"source_line":862,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":12360,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14675,"source_line":875,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":17912,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$11::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":17464,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":25315,"source_line":1069,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":17612,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":12264,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":4908,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":3320,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":3896,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3870,"record_offset":9760,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3870,"record_offset":10120,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprints::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":3870,"record_offset":10752,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21715,"source_line":260,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":15656,"parent_record_offset":15556,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":15120,"parent_record_offset":15088,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":15556,"parent_record_offset":15528,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":15088,"parent_record_offset":15056,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3870,"record_offset":15748,"parent_record_offset":15724,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::IntoIter::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3870,"record_offset":19660,"parent_record_offset":19632,"ancestor_record_offset":3060,"inlinee":13146,"source_line":227,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3870,"record_offset":15384,"parent_record_offset":15236,"ancestor_record_offset":3060,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":3870,"record_offset":18884,"parent_record_offset":18856,"ancestor_record_offset":3060,"inlinee":11998,"source_line":356,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5328,"parent_record_offset":5300,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5268,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5300,"parent_record_offset":5268,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5472,"parent_record_offset":5300,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3870,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":3870,"record_offset":19712,"parent_record_offset":19684,"ancestor_record_offset":3060,"inlinee":8568,"source_line":219,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3870,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":3060,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3870,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":3060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3870,"record_offset":9180,"parent_record_offset":9148,"ancestor_record_offset":3060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3870,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":3060,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3870,"record_offset":9244,"parent_record_offset":9208,"ancestor_record_offset":3060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3870,"record_offset":12060,"parent_record_offset":11808,"ancestor_record_offset":3060,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3870,"record_offset":12732,"parent_record_offset":12676,"ancestor_record_offset":3060,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3870,"record_offset":11976,"parent_record_offset":11808,"ancestor_record_offset":3060,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3870,"record_offset":11864,"parent_record_offset":11808,"ancestor_record_offset":3060,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3870,"record_offset":12596,"parent_record_offset":12408,"ancestor_record_offset":3060,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3870,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":3060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3870,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":3060,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3870,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":3060,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3870,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":3060,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":3870,"record_offset":15272,"parent_record_offset":15236,"ancestor_record_offset":3060,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3870,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3870,"record_offset":15620,"parent_record_offset":15588,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3870,"record_offset":3532,"parent_record_offset":3436,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3870,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":3060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":17524,"parent_record_offset":17464,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":17688,"parent_record_offset":17612,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":12296,"parent_record_offset":12264,"ancestor_record_offset":3060,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":18092,"parent_record_offset":17968,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3870,"record_offset":8840,"parent_record_offset":8776,"ancestor_record_offset":3060,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3870,"record_offset":9516,"parent_record_offset":9492,"ancestor_record_offset":3060,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3870,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":3060,"inlinee":6226,"source_line":46,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3870,"record_offset":15356,"parent_record_offset":15236,"ancestor_record_offset":3060,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3870,"record_offset":15236,"parent_record_offset":15212,"ancestor_record_offset":3060,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3870,"record_offset":15296,"parent_record_offset":15272,"ancestor_record_offset":3060,"inlinee":13144,"source_line":160,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":3870,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":3060,"inlinee":13128,"source_line":11,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":8812,"parent_record_offset":8776,"ancestor_record_offset":3060,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":5160,"parent_record_offset":5132,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":9612,"parent_record_offset":8776,"ancestor_record_offset":3060,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":4492,"parent_record_offset":4032,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":5208,"parent_record_offset":5028,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":9672,"parent_record_offset":8752,"ancestor_record_offset":3060,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3870,"record_offset":3844,"parent_record_offset":3816,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":5536,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3870,"record_offset":15828,"parent_record_offset":15804,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3870,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":3060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3870,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":3060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":4452,"parent_record_offset":4420,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3870,"record_offset":5936,"parent_record_offset":4988,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":15408,"parent_record_offset":15384,"ancestor_record_offset":3060,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":15972,"parent_record_offset":15944,"ancestor_record_offset":3060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":15432,"parent_record_offset":15408,"ancestor_record_offset":3060,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":3060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3870,"record_offset":15804,"parent_record_offset":15780,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":4548,"parent_record_offset":4524,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":4196,"parent_record_offset":4088,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3870,"record_offset":3436,"parent_record_offset":3408,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3870,"record_offset":3928,"parent_record_offset":3896,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3870,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3870,"record_offset":4032,"parent_record_offset":4000,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3870,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3870,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3870,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3870,"record_offset":4524,"parent_record_offset":4000,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3792,"parent_record_offset":3632,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3352,"parent_record_offset":3320,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3870,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3870,"record_offset":4088,"parent_record_offset":4060,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3870,"record_offset":4336,"parent_record_offset":4088,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3870,"record_offset":4308,"parent_record_offset":4088,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3870,"record_offset":4140,"parent_record_offset":4088,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3870,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":3060,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$4::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":3870,"record_offset":9540,"parent_record_offset":9516,"ancestor_record_offset":3060,"inlinee":25252,"source_line":180,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":11808,"parent_record_offset":11780,"ancestor_record_offset":3060,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":3060,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":12408,"parent_record_offset":12384,"ancestor_record_offset":3060,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3870,"record_offset":12852,"parent_record_offset":12408,"ancestor_record_offset":3060,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":18152,"parent_record_offset":17968,"ancestor_record_offset":3060,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":17968,"parent_record_offset":17912,"ancestor_record_offset":3060,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3870,"record_offset":12480,"parent_record_offset":12452,"ancestor_record_offset":3060,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3870,"record_offset":12452,"parent_record_offset":12408,"ancestor_record_offset":3060,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":3870,"record_offset":11536,"parent_record_offset":11512,"ancestor_record_offset":3060,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":3060,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":3060,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":9808,"parent_record_offset":9784,"ancestor_record_offset":3060,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":11512,"parent_record_offset":11292,"ancestor_record_offset":3060,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":3060,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3870,"record_offset":10856,"parent_record_offset":10824,"ancestor_record_offset":3060,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":3060,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11400,"parent_record_offset":11292,"ancestor_record_offset":3060,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":3060,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":9280,"parent_record_offset":8972,"ancestor_record_offset":3060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11720,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":9916,"parent_record_offset":9856,"ancestor_record_offset":3060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":8972,"parent_record_offset":8944,"ancestor_record_offset":3060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11316,"parent_record_offset":11292,"ancestor_record_offset":3060,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":10908,"parent_record_offset":10884,"ancestor_record_offset":3060,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":10884,"parent_record_offset":10824,"ancestor_record_offset":3060,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":9832,"parent_record_offset":9808,"ancestor_record_offset":3060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":9888,"parent_record_offset":9856,"ancestor_record_offset":3060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":10932,"parent_record_offset":10908,"ancestor_record_offset":3060,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":3060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3870,"record_offset":9052,"parent_record_offset":9020,"ancestor_record_offset":3060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3870,"record_offset":9084,"parent_record_offset":9052,"ancestor_record_offset":3060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3870,"record_offset":10376,"parent_record_offset":10248,"ancestor_record_offset":3060,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3870,"record_offset":9020,"parent_record_offset":8972,"ancestor_record_offset":3060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":10976,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":3060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3870,"record_offset":11292,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3870,"record_offset":11004,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3870,"record_offset":9148,"parent_record_offset":8972,"ancestor_record_offset":3060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3870,"record_offset":11088,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3870,"record_offset":11628,"parent_record_offset":10932,"ancestor_record_offset":3060,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3870,"record_offset":9208,"parent_record_offset":9148,"ancestor_record_offset":3060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":4948,"parent_record_offset":4908,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3870,"record_offset":5072,"parent_record_offset":5028,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":5236,"parent_record_offset":5028,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3870,"record_offset":5508,"parent_record_offset":5236,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3870,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":3060,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3870,"record_offset":9784,"parent_record_offset":9760,"ancestor_record_offset":3060,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3870,"record_offset":8896,"parent_record_offset":8868,"ancestor_record_offset":3060,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":18312,"parent_record_offset":18152,"ancestor_record_offset":3060,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3870,"record_offset":5100,"parent_record_offset":5072,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3870,"record_offset":5132,"parent_record_offset":5100,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3870,"record_offset":5028,"parent_record_offset":4988,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3870,"record_offset":4988,"parent_record_offset":4948,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect
// 记录形式: compiled_procedure; instances=1
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3870,"record_offset":20496,"rva":29827392,"va":5398536512}
// 编译实例 RVAs: 0x1c72140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25788421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8fb7f76fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398538128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25787623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8198376fe","size":5,"soft_fail":false,"target_va":5372749744,"va":5398537362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4996927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8c1c0b3ff","size":5,"soft_fail":false,"target_va":5393539792,"va":5398536714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c3a540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-228506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e86683fcff","size":5,"soft_fail":false,"target_va":5398308160,"va":5398536661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c3b7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-224360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e89893fcff","size":5,"soft_fail":false,"target_va":5398312928,"va":5398537283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4439824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e810bf4300","size":5,"soft_fail":false,"target_va":5402978144,"va":5398538315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-27232343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8a97760fe","size":5,"soft_fail":false,"target_va":5371304880,"va":5398537218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-21011185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e80f65bffe","size":5,"soft_fail":false,"target_va":5377527120,"va":5398538300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4441084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8fcc34300","size":5,"soft_fail":false,"target_va":5402979344,"va":5398538255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24028939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398536512,"raw_hex":"e8f55891fe","size":5,"soft_fail":false,"target_va":5374508912,"va":5398537846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3870,"record_offset":20732,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":20788,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":21060,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":22064,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":22676,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":21660,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3870,"record_offset":20900,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":3870,"record_offset":21196,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":24156,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":22868,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":20816,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3870,"record_offset":21756,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":22612,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":21840,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3870,"record_offset":23164,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":23708,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":21472,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":22395,"source_line":469,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":23472,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":23380,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":23352,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":21632,"parent_record_offset":20496,"ancestor_record_offset":20496,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":22264,"parent_record_offset":22240,"ancestor_record_offset":20496,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":22288,"parent_record_offset":22264,"ancestor_record_offset":20496,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":20496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":22240,"parent_record_offset":22148,"ancestor_record_offset":20496,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3870,"record_offset":24396,"parent_record_offset":24288,"ancestor_record_offset":20496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":24324,"parent_record_offset":24288,"ancestor_record_offset":20496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":21132,"parent_record_offset":21100,"ancestor_record_offset":20496,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":22148,"parent_record_offset":22120,"ancestor_record_offset":20496,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":20496,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":22328,"parent_record_offset":22120,"ancestor_record_offset":20496,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3870,"record_offset":24288,"parent_record_offset":24256,"ancestor_record_offset":20496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3870,"record_offset":20756,"parent_record_offset":20732,"ancestor_record_offset":20496,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":21100,"parent_record_offset":21060,"ancestor_record_offset":20496,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3870,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":20496,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":3870,"record_offset":22092,"parent_record_offset":22064,"ancestor_record_offset":20496,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3870,"record_offset":22184,"parent_record_offset":22148,"ancestor_record_offset":20496,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":3870,"record_offset":22208,"parent_record_offset":22184,"ancestor_record_offset":20496,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3870,"record_offset":23116,"parent_record_offset":23092,"ancestor_record_offset":20496,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3870,"record_offset":21384,"parent_record_offset":21196,"ancestor_record_offset":20496,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3870,"record_offset":21228,"parent_record_offset":21196,"ancestor_record_offset":20496,"inlinee":27298,"source_line":60,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3870,"record_offset":24360,"parent_record_offset":24324,"ancestor_record_offset":20496,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":23644,"parent_record_offset":23500,"ancestor_record_offset":20496,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":21536,"parent_record_offset":21496,"ancestor_record_offset":20496,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3870,"record_offset":21352,"parent_record_offset":21228,"ancestor_record_offset":20496,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3870,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":20496,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":24204,"parent_record_offset":24180,"ancestor_record_offset":20496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3870,"record_offset":20864,"parent_record_offset":20840,"ancestor_record_offset":20496,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":21020,"parent_record_offset":20996,"ancestor_record_offset":20496,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":21292,"parent_record_offset":21256,"ancestor_record_offset":20496,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3870,"record_offset":20968,"parent_record_offset":20932,"ancestor_record_offset":20496,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3870,"record_offset":21164,"parent_record_offset":21060,"ancestor_record_offset":20496,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3870,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":20496,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3870,"record_offset":23068,"parent_record_offset":22916,"ancestor_record_offset":20496,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":22704,"parent_record_offset":22676,"ancestor_record_offset":20496,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":21688,"parent_record_offset":21660,"ancestor_record_offset":20496,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":22732,"parent_record_offset":22704,"ancestor_record_offset":20496,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3870,"record_offset":21716,"parent_record_offset":21688,"ancestor_record_offset":20496,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3870,"record_offset":20840,"parent_record_offset":20816,"ancestor_record_offset":20496,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":20996,"parent_record_offset":20932,"ancestor_record_offset":20496,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3870,"record_offset":21780,"parent_record_offset":21756,"ancestor_record_offset":20496,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":21920,"parent_record_offset":21840,"ancestor_record_offset":20496,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":22916,"parent_record_offset":22892,"ancestor_record_offset":20496,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":20496,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3870,"record_offset":21256,"parent_record_offset":21228,"ancestor_record_offset":20496,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":3870,"record_offset":23092,"parent_record_offset":23068,"ancestor_record_offset":20496,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3870,"record_offset":23280,"parent_record_offset":23192,"ancestor_record_offset":20496,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3870,"record_offset":23192,"parent_record_offset":23164,"ancestor_record_offset":20496,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":21564,"parent_record_offset":21496,"ancestor_record_offset":20496,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":23536,"parent_record_offset":23500,"ancestor_record_offset":20496,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":23500,"parent_record_offset":23472,"ancestor_record_offset":20496,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":20496,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":23312,"parent_record_offset":23280,"ancestor_record_offset":20496,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B47A
// 首条位置: {"module":3838,"inlinee":111738}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$28::cmp
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3838,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2524,"inlinee":111740,"source_line":84,"archive":"_pdb/A/modules/03838.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3838,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2524,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/03838.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B47D
// 首条位置: {"module":3838,"inlinee":111741}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3838,"record_offset":12236,"parent_record_offset":12208,"ancestor_record_offset":12208,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03838.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::clothline
// 记录形式: compiled_procedure; instances=1
// 源码行: 399
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3870,"record_offset":25588,"rva":29829808,"va":5398538928}
// 编译实例 RVAs: 0x1c72ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141c38fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-238517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398538928,"raw_hex":"e84b5cfcff","size":5,"soft_fail":false,"target_va":5398302656,"va":5398541168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag_connections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5054782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398538928,"raw_hex":"e83e214d00","size":5,"soft_fail":false,"target_va":5403593968,"va":5398539181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3870,"record_offset":25996,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3870,"record_offset":26428,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":27888,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":27300,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":25784,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":26028,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":27044,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":26808,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":26752,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3870,"record_offset":26248,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":25968,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::flag_connections::clothline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag_connections.rs
//     evidence={"module":3870,"record_offset":26368,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":111931,"source_line":419,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":3870,"record_offset":28264,"parent_record_offset":25588,"ancestor_record_offset":25588,"inlinee":60169,"source_line":49,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3870,"record_offset":26548,"parent_record_offset":26512,"ancestor_record_offset":25588,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3870,"record_offset":26512,"parent_record_offset":26484,"ancestor_record_offset":25588,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3870,"record_offset":26484,"parent_record_offset":26456,"ancestor_record_offset":25588,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":26148,"parent_record_offset":26064,"ancestor_record_offset":25588,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":26116,"parent_record_offset":26064,"ancestor_record_offset":25588,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3870,"record_offset":25840,"parent_record_offset":25808,"ancestor_record_offset":25588,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3870,"record_offset":26580,"parent_record_offset":26512,"ancestor_record_offset":25588,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":3870,"record_offset":28032,"parent_record_offset":27924,"ancestor_record_offset":25588,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3870,"record_offset":27924,"parent_record_offset":27888,"ancestor_record_offset":25588,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":25896,"parent_record_offset":25808,"ancestor_record_offset":25588,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":27108,"parent_record_offset":27072,"ancestor_record_offset":25588,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":27072,"parent_record_offset":27044,"ancestor_record_offset":25588,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":26064,"parent_record_offset":26028,"ancestor_record_offset":25588,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":25588,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3870,"record_offset":26652,"parent_record_offset":26624,"ancestor_record_offset":25588,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3870,"record_offset":26456,"parent_record_offset":26428,"ancestor_record_offset":25588,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":26396,"parent_record_offset":26368,"ancestor_record_offset":25588,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":25928,"parent_record_offset":25896,"ancestor_record_offset":25588,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03870.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag_connections::clothline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 419
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B53B
// 首条位置: {"module":3870,"inlinee":111931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3870,"record_offset":26396,"parent_record_offset":26368,"ancestor_record_offset":26368,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/03870.jsonl.gz"}
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
// 完整结构及原始类型记录: flag_connections.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::decorator_visual::flag_connections::CandidateInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::vec::impl$19::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>", "system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>", "alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["system_decorator::decorator_visual::flag_connections::CandidateInfo", "alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flag_connections.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flag_connections.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::clothline
// 信息 ID: d3be36d1eef238f2d71af37b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x1c6add5 -> 0x1c72ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x1c7158b -> 0x1c72ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x1c71662 -> 0x1c72ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::clothline::closure$0
// 信息 ID: d5d85d3661bc64a620babfcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::flag_connections::clothline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag_connections::clothline; module=3870, inline_record=26368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::generate_flag_connections
// 信息 ID: a2db019bbeb6d48a465984fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e0f99e25d3a57682c5c2e7cd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)> > @ 0x17c027d -> 0x1c6ff20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)> @ 0x1c84043 -> 0x1c6ff20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect
// 信息 ID: 733d3abf8a45dd00927e064f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::generate_clothline::generate_clothline_visuals @ 0x1c69080 -> 0x1c72140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::flag_connections::generate_flag_connections @ 0x1c70ef0 -> 0x1c72140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$0
// 信息 ID: 9b0d04c7f2054b67e10904d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::flag_connections::greedily_connect", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0 <- core::slice::sort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; module=3838, inline_record=2524; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; module=3838, inline_record=2636; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; module=3838, inline_record=2748; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; module=3838, inline_record=3776; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0>,alloc::slice::stable_sort::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$1<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$2<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >,alloc::slice::stable_sort::closure_env$3<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> > >; module=3838, inline_record=4116; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=3838, inline_record=19720; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc::slice::impl$0::sort_by::closure_env$0<system_decorator::decorator_visual::flag_connections::CandidateInfo,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$0> >; module=3838, inline_record=20016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag_connections::greedily_connect::closure$1
// 信息 ID: 246e78625dd21fe02d3a9242
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::flag_connections::greedily_connect", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1 <- core::slice::sort::insertion_sort_shift_left<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; module=3838, inline_record=12208; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; module=3838, inline_record=12340; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; module=3838, inline_record=12452; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; module=3838, inline_record=13488; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1>,alloc::slice::stable_sort::closure_env$0<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> > >; module=3838, inline_record=13828; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=3838, inline_record=18356; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<usize,f32>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<usize,f32>,system_decorator::decorator_visual::flag_connections::greedily_connect::closure_env$1> >; module=3838, inline_record=18776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
