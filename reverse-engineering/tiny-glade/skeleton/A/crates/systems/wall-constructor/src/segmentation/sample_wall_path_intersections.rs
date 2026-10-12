// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 533f8d89e2b6f4f475c7a46f333398606727f85dedb54652a05c0d9708da445a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sample_wall_path_intersections.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 364
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
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":364,"record_offset":3068,"rva":5112944,"va":5373822064}
// 编译实例 RVAs: 0x4e0470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4209713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8cfc3bfff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373823228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1076214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e80a94efff","size":5,"soft_fail":false,"target_va":5372749696,"va":5373825905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1080421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e89b83efff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373830128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-673008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e810bbf5ff","size":5,"soft_fail":false,"target_va":5373149840,"va":5373822843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8201576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e868257d00","size":5,"soft_fail":false,"target_va":5382031664,"va":5373830083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29769808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e85040c601","size":5,"soft_fail":false,"target_va":5403593392,"va":5373823579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29769773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e82d40c601","size":5,"soft_fail":false,"target_va":5403594064,"va":5373824286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29147343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8cfc0bc01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373830796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29095002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e85af4bb01","size":5,"soft_fail":false,"target_va":5402918528,"va":5373823521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t-2520376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8c88ad9ff","size":5,"soft_fail":false,"target_va":5371306160,"va":5373826531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3696341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8d5663800","size":5,"soft_fail":false,"target_va":5377527120,"va":5373830774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29147858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8d2c2bc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373830825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29148165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e805c4bc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373830710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29148748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e84cc6bc01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373830591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29148172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e80cc4bc01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373830767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e81ddaffff","size":5,"soft_fail":false,"target_va":5373820784,"va":5373830478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29157493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e875e8bc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373822630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e82afbffff","size":5,"soft_fail":false,"target_va":5373820976,"va":5373822209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29158365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8ddebbc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373822286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2534110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8deaa2600","size":5,"soft_fail":false,"target_va":5376356912,"va":5373822797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2537994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e80aba2600","size":5,"soft_fail":false,"target_va":5376360560,"va":5373822561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2533950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e83eaa2600","size":5,"soft_fail":false,"target_va":5376356144,"va":5373822189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8042838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e856b97a00","size":5,"soft_fail":false,"target_va":5381865424,"va":5373822581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t3115943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373822064,"raw_hex":"e8a78b2f00","size":5,"soft_fail":false,"target_va":5376939952,"va":5373824004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":21836,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":6084,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":18168,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":22812,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":21864,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":364,"record_offset":5996,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":364,"record_offset":6112,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":364,"record_offset":16856,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":21808,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":364,"record_offset":23312,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":364,"record_offset":22700,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":18820,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":364,"record_offset":20004,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":23284,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":23064,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":364,"record_offset":22920,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":364,"record_offset":6056,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":18732,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":18644,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":20128,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":4948,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":3360,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":3936,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":364,"record_offset":17168,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":20100,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":6660,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21419,"source_line":133,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":19060,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":3068,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":3068,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":15716,"parent_record_offset":15692,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7224,"parent_record_offset":7116,"ancestor_record_offset":3068,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":15692,"parent_record_offset":15592,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":18344,"parent_record_offset":18320,"ancestor_record_offset":3068,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":15624,"parent_record_offset":15592,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":18320,"parent_record_offset":18292,"ancestor_record_offset":3068,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":3068,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":3068,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7076,"parent_record_offset":7036,"ancestor_record_offset":3068,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7344,"parent_record_offset":7076,"ancestor_record_offset":3068,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":15592,"parent_record_offset":15564,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":3068,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":18224,"parent_record_offset":18168,"ancestor_record_offset":3068,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":18196,"parent_record_offset":18168,"ancestor_record_offset":3068,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7700,"parent_record_offset":7672,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":18268,"parent_record_offset":18224,"ancestor_record_offset":3068,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":6996,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7036,"parent_record_offset":6996,"ancestor_record_offset":3068,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":15272,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":22352,"parent_record_offset":22308,"ancestor_record_offset":3068,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":364,"record_offset":18608,"parent_record_offset":18224,"ancestor_record_offset":3068,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":364,"record_offset":22092,"parent_record_offset":21912,"ancestor_record_offset":3068,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":364,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":3068,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":364,"record_offset":21888,"parent_record_offset":21864,"ancestor_record_offset":3068,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":364,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":3068,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":364,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":3068,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":364,"record_offset":6176,"parent_record_offset":6136,"ancestor_record_offset":3068,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":364,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":3068,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":364,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":3068,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":364,"record_offset":7160,"parent_record_offset":7116,"ancestor_record_offset":3068,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":364,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":3068,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5348,"parent_record_offset":5320,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5288,"parent_record_offset":5260,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5492,"parent_record_offset":5320,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":364,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":364,"record_offset":8552,"parent_record_offset":8516,"ancestor_record_offset":3068,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":364,"record_offset":18516,"parent_record_offset":18488,"ancestor_record_offset":3068,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":364,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":364,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":3068,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":364,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11392,"parent_record_offset":11360,"ancestor_record_offset":3068,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11448,"parent_record_offset":11360,"ancestor_record_offset":3068,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":364,"record_offset":17444,"parent_record_offset":17420,"ancestor_record_offset":3068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":364,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":3068,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":364,"record_offset":16908,"parent_record_offset":16880,"ancestor_record_offset":3068,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":364,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":3068,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":364,"record_offset":22264,"parent_record_offset":22168,"ancestor_record_offset":3068,"inlinee":17354,"source_line":32,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":364,"record_offset":8076,"parent_record_offset":8040,"ancestor_record_offset":3068,"inlinee":21435,"source_line":97,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":364,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":3068,"inlinee":17352,"source_line":73,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":364,"record_offset":18884,"parent_record_offset":18848,"ancestor_record_offset":3068,"inlinee":8551,"source_line":133,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":364,"record_offset":7880,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":3068,"inlinee":8555,"source_line":636,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8004,"parent_record_offset":7952,"ancestor_record_offset":3068,"inlinee":21434,"source_line":107,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":7916,"parent_record_offset":7880,"ancestor_record_offset":3068,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8040,"parent_record_offset":8004,"ancestor_record_offset":3068,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":7952,"parent_record_offset":7916,"ancestor_record_offset":3068,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8112,"parent_record_offset":8076,"ancestor_record_offset":3068,"inlinee":8553,"source_line":613,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":364,"record_offset":18488,"parent_record_offset":18460,"ancestor_record_offset":3068,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":364,"record_offset":18460,"parent_record_offset":18224,"ancestor_record_offset":3068,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":364,"record_offset":22120,"parent_record_offset":21912,"ancestor_record_offset":3068,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":364,"record_offset":22308,"parent_record_offset":22264,"ancestor_record_offset":3068,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":364,"record_offset":5424,"parent_record_offset":5400,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":364,"record_offset":6248,"parent_record_offset":6200,"ancestor_record_offset":3068,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":364,"record_offset":22500,"parent_record_offset":22168,"ancestor_record_offset":3068,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":364,"record_offset":15656,"parent_record_offset":15624,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":364,"record_offset":3572,"parent_record_offset":3476,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":364,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8396,"parent_record_offset":8300,"ancestor_record_offset":3068,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8452,"parent_record_offset":8300,"ancestor_record_offset":3068,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":3068,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8340,"parent_record_offset":8300,"ancestor_record_offset":3068,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":3068,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":5184,"parent_record_offset":5156,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":4532,"parent_record_offset":4072,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":5232,"parent_record_offset":5056,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":16988,"parent_record_offset":16908,"ancestor_record_offset":3068,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8580,"parent_record_offset":8552,"ancestor_record_offset":3068,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":3068,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":364,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":5556,"parent_record_offset":5260,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":364,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":364,"record_offset":17864,"parent_record_offset":17840,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":364,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":4492,"parent_record_offset":4460,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":3068,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":5448,"parent_record_offset":5424,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":364,"record_offset":5956,"parent_record_offset":5020,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":364,"record_offset":3508,"parent_record_offset":3476,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":364,"record_offset":6504,"parent_record_offset":6200,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":364,"record_offset":22844,"parent_record_offset":22812,"ancestor_record_offset":3068,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":364,"record_offset":3536,"parent_record_offset":3508,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":364,"record_offset":22876,"parent_record_offset":22844,"ancestor_record_offset":3068,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":364,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":3068,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":364,"record_offset":6352,"parent_record_offset":6312,"ancestor_record_offset":3068,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":364,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":15392,"parent_record_offset":15300,"ancestor_record_offset":3068,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":3068,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":22168,"parent_record_offset":22144,"ancestor_record_offset":3068,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":17064,"parent_record_offset":17012,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":4236,"parent_record_offset":4128,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":15300,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":364,"record_offset":3476,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":364,"record_offset":3968,"parent_record_offset":3936,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":364,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":364,"record_offset":4072,"parent_record_offset":4040,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":364,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":364,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":364,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":364,"record_offset":4564,"parent_record_offset":4040,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3672,"parent_record_offset":3640,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3832,"parent_record_offset":3672,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3392,"parent_record_offset":3360,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3448,"parent_record_offset":3416,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":364,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":364,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":364,"record_offset":4376,"parent_record_offset":4128,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":364,"record_offset":4348,"parent_record_offset":4128,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":364,"record_offset":4180,"parent_record_offset":4128,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":364,"record_offset":17976,"parent_record_offset":17952,"ancestor_record_offset":3068,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=20
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":364,"record_offset":11700,"parent_record_offset":11664,"ancestor_record_offset":3068,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11604,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11336,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":3068,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11248,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11972,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11160,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11512,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":10648,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":10760,"parent_record_offset":10672,"ancestor_record_offset":3068,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8276,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":3068,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8640,"parent_record_offset":8608,"ancestor_record_offset":3068,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":364,"record_offset":11636,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":364,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":3068,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":364,"record_offset":17216,"parent_record_offset":17192,"ancestor_record_offset":3068,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":364,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":3068,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":364,"record_offset":17904,"parent_record_offset":17684,"ancestor_record_offset":3068,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":364,"record_offset":17248,"parent_record_offset":17216,"ancestor_record_offset":3068,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":3068,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17792,"parent_record_offset":17684,"ancestor_record_offset":3068,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17732,"parent_record_offset":17708,"ancestor_record_offset":3068,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":18112,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17708,"parent_record_offset":17684,"ancestor_record_offset":3068,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":3068,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17276,"parent_record_offset":17216,"ancestor_record_offset":3068,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":3068,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":364,"record_offset":17564,"parent_record_offset":17536,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":364,"record_offset":17592,"parent_record_offset":17564,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":364,"record_offset":17652,"parent_record_offset":17536,"ancestor_record_offset":3068,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":364,"record_offset":17536,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17368,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":364,"record_offset":17684,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":364,"record_offset":17396,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":364,"record_offset":17480,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":364,"record_offset":18020,"parent_record_offset":17324,"ancestor_record_offset":3068,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":4984,"parent_record_offset":4948,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":364,"record_offset":5096,"parent_record_offset":5056,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":5260,"parent_record_offset":5056,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":364,"record_offset":5528,"parent_record_offset":5260,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":8248,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":15480,"parent_record_offset":15452,"ancestor_record_offset":3068,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":10792,"parent_record_offset":10760,"ancestor_record_offset":3068,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":364,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":364,"record_offset":5156,"parent_record_offset":5124,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":364,"record_offset":5056,"parent_record_offset":5020,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":364,"record_offset":5020,"parent_record_offset":4984,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":7672,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":21433,"source_line":266,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":15452,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":21440,"source_line":283,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":19020,"parent_record_offset":18884,"ancestor_record_offset":3068,"inlinee":21421,"source_line":55,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=20
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":11664,"parent_record_offset":11076,"ancestor_record_offset":3068,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":11076,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53AD
// 首条位置: {"module":364,"inlinee":21421}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53AB
// 首条位置: {"module":364,"inlinee":21419}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":6996,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":15272,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":364,"record_offset":7880,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":15516,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":15300,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":10648,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8276,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":8248,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":7672,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":21433,"source_line":266,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":364,"record_offset":15452,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":21440,"source_line":283,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":11076,"parent_record_offset":6660,"ancestor_record_offset":6660,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":6660,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6660,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":15716,"parent_record_offset":15692,"ancestor_record_offset":6660,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":7224,"parent_record_offset":7116,"ancestor_record_offset":6660,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":364,"record_offset":15692,"parent_record_offset":15592,"ancestor_record_offset":6660,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":15624,"parent_record_offset":15592,"ancestor_record_offset":6660,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":6660,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7116,"parent_record_offset":7076,"ancestor_record_offset":6660,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7076,"parent_record_offset":7036,"ancestor_record_offset":6660,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7344,"parent_record_offset":7076,"ancestor_record_offset":6660,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":15592,"parent_record_offset":15564,"ancestor_record_offset":6660,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":6660,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7700,"parent_record_offset":7672,"ancestor_record_offset":6660,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7036,"parent_record_offset":6996,"ancestor_record_offset":6660,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":364,"record_offset":7160,"parent_record_offset":7116,"ancestor_record_offset":6660,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":364,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":6660,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":364,"record_offset":8552,"parent_record_offset":8516,"ancestor_record_offset":6660,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11392,"parent_record_offset":11360,"ancestor_record_offset":6660,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":364,"record_offset":11448,"parent_record_offset":11360,"ancestor_record_offset":6660,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":364,"record_offset":8076,"parent_record_offset":8040,"ancestor_record_offset":6660,"inlinee":21435,"source_line":97,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":6660,"inlinee":8555,"source_line":636,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8004,"parent_record_offset":7952,"ancestor_record_offset":6660,"inlinee":21434,"source_line":107,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":7916,"parent_record_offset":7880,"ancestor_record_offset":6660,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8040,"parent_record_offset":8004,"ancestor_record_offset":6660,"inlinee":6062,"source_line":268,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":7952,"parent_record_offset":7916,"ancestor_record_offset":6660,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":364,"record_offset":8112,"parent_record_offset":8076,"ancestor_record_offset":6660,"inlinee":8553,"source_line":613,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":364,"record_offset":15656,"parent_record_offset":15624,"ancestor_record_offset":6660,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8396,"parent_record_offset":8300,"ancestor_record_offset":6660,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8452,"parent_record_offset":8300,"ancestor_record_offset":6660,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":11188,"parent_record_offset":11160,"ancestor_record_offset":6660,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":364,"record_offset":8340,"parent_record_offset":8300,"ancestor_record_offset":6660,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8580,"parent_record_offset":8552,"ancestor_record_offset":6660,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":6660,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":6660,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":364,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":6660,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":15416,"parent_record_offset":15392,"ancestor_record_offset":6660,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":364,"record_offset":15360,"parent_record_offset":15300,"ancestor_record_offset":6660,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":6660,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":7844,"parent_record_offset":7760,"ancestor_record_offset":6660,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":15392,"parent_record_offset":15300,"ancestor_record_offset":6660,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":364,"record_offset":8160,"parent_record_offset":8136,"ancestor_record_offset":6660,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":364,"record_offset":11700,"parent_record_offset":11664,"ancestor_record_offset":6660,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11604,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11336,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":6660,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11248,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11972,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11160,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":364,"record_offset":11512,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":10760,"parent_record_offset":10672,"ancestor_record_offset":6660,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":6660,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8640,"parent_record_offset":8608,"ancestor_record_offset":6660,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":364,"record_offset":11636,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":15480,"parent_record_offset":15452,"ancestor_record_offset":6660,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":10792,"parent_record_offset":10760,"ancestor_record_offset":6660,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":364,"record_offset":11664,"parent_record_offset":11076,"ancestor_record_offset":6660,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53B9
// 首条位置: {"module":364,"inlinee":21433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7700,"parent_record_offset":7672,"ancestor_record_offset":7672,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":364,"record_offset":8552,"parent_record_offset":8516,"ancestor_record_offset":8516,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":364,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":7736,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":364,"record_offset":7788,"parent_record_offset":7760,"ancestor_record_offset":7736,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8580,"parent_record_offset":8552,"ancestor_record_offset":8516,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":364,"record_offset":8608,"parent_record_offset":8580,"ancestor_record_offset":8516,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":364,"record_offset":7844,"parent_record_offset":7760,"ancestor_record_offset":7736,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$48::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":364,"record_offset":8640,"parent_record_offset":8608,"ancestor_record_offset":8516,"inlinee":21438,"source_line":18,"archive":"_pdb/A/modules/00364.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x53C0
// 首条位置: {"module":364,"inlinee":21440}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":364,"record_offset":15480,"parent_record_offset":15452,"ancestor_record_offset":15452,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00364.jsonl.gz"}
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
// 完整结构及原始类型记录: sample_wall_path_intersections.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sample_wall_path_intersections.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production-authority/three-version-path-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sample_wall_path_intersections.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
// 信息 ID: 7312a34fccfaf5f8e98ba05d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=7672; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=7736; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=8516; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=9188; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=9616; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=9976; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=10312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=12032; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
// 信息 ID: b28e19aecee71a2b0d206089
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=15452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections
// 信息 ID: 80bc723b08323256d9d2580f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 605f25e380b3102a4116653d: void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xaf933 -> 0x4e0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x2f6ac1 -> 0x4e0470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
// 信息 ID: 3c2ed9724c93d06269b28e5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=19020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask
// 信息 ID: 1a2cff46dc79ae95d78bee87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=364, inline_record=6660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
