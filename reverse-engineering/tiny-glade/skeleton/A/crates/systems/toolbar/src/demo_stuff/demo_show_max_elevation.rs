// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/demo_stuff/demo_show_max_elevation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 30f56a81d071b9d6b5619bf112831a5583f4f2e7cbe95d90e7eccc02f9330577
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: demo_show_max_elevation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2468, 3645
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3645,"record_offset":3052,"rva":28685984,"va":5397395104}
// 编译实例 RVAs: 0x1b5b6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27779392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8c01e58fe","size":5,"soft_fail":false,"target_va":5369617648,"va":5397397035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24646769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e88feb87fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5397396460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24647551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e881e887fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5397397258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043d460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24242814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e882158efe","size":5,"soft_fail":false,"target_va":5373154400,"va":5397397209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21516397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e893afb7fe","size":5,"soft_fail":false,"target_va":5375881440,"va":5397397832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20452150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8caecc7fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5397396145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14161ef80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5492596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e88c30acff","size":5,"soft_fail":false,"target_va":5391904640,"va":5397397231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5580723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8b3275500","size":5,"soft_fail":false,"target_va":5402978144,"va":5397397416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-26088293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e89bec71fe","size":5,"soft_fail":false,"target_va":5371308800,"va":5397397088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5581282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8e2295500","size":5,"soft_fail":false,"target_va":5402978688,"va":5397397401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5581952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8802c5500","size":5,"soft_fail":false,"target_va":5402979344,"va":5397397387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e862f2ffff","size":5,"soft_fail":false,"target_va":5397393824,"va":5397397305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5584451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e843365500","size":5,"soft_fail":false,"target_va":5402980128,"va":5397395672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e82cfbffff","size":5,"soft_fail":false,"target_va":5397394016,"va":5397395247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5585327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8af395500","size":5,"soft_fail":false,"target_va":5402980656,"va":5397395324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22996063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8a11ba1fe","size":5,"soft_fail":false,"target_va":5374399152,"va":5397395210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15525987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e89d1713ff","size":5,"soft_fail":false,"target_va":5381870304,"va":5397396286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21983950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8328db0fe","size":5,"soft_fail":false,"target_va":5375412208,"va":5397396153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21672234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8d64eb5fe","size":5,"soft_fail":false,"target_va":5375723824,"va":5397396053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-13195948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e854a536ff","size":5,"soft_fail":false,"target_va":5384200608,"va":5397396551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-13196136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e898a436ff","size":5,"soft_fail":false,"target_va":5384200400,"va":5397396531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-13195921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e86fa536ff","size":5,"soft_fail":false,"target_va":5384200656,"va":5397396572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-22034847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e861c6affe","size":5,"soft_fail":false,"target_va":5375361328,"va":5397396170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21038934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8aaf8befe","size":5,"soft_fail":false,"target_va":5376356912,"va":5397395841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21035048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8d807bffe","size":5,"soft_fail":false,"target_va":5376360560,"va":5397395603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21039084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e814f8befe","size":5,"soft_fail":false,"target_va":5376356144,"va":5397395223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h6a28a4a69af7475d (.llvm.7250520506814577795)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17773644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e8b4cbf0fe","size":5,"soft_fail":false,"target_va":5379621984,"va":5397395623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24237675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e895298efe","size":5,"soft_fail":false,"target_va":5373159392,"va":5397397062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_side
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14234604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397395104,"raw_hex":"e814cc26ff","size":5,"soft_fail":false,"target_va":5383161616,"va":5397396215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":7160,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3645,"record_offset":7312,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3645,"record_offset":6772,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3645,"record_offset":6868,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3645,"record_offset":8336,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3645,"record_offset":5928,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3645,"record_offset":6544,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3645,"record_offset":8624,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3645,"record_offset":6980,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3645,"record_offset":7044,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3645,"record_offset":7072,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3645,"record_offset":7788,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3645,"record_offset":7548,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":4860,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":3272,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":3848,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3645,"record_offset":8396,"parent_record_offset":3052,"ancestor_record_offset":3052,"inlinee":50927,"source_line":360,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":3052,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":3052,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":8812,"parent_record_offset":8788,"ancestor_record_offset":3052,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":7192,"parent_record_offset":7160,"ancestor_record_offset":3052,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3645,"record_offset":8788,"parent_record_offset":8696,"ancestor_record_offset":3052,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3645,"record_offset":8724,"parent_record_offset":8696,"ancestor_record_offset":3052,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3645,"record_offset":8552,"parent_record_offset":8520,"ancestor_record_offset":3052,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3645,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":3052,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3645,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":3052,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3645,"record_offset":8520,"parent_record_offset":8484,"ancestor_record_offset":3052,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3645,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":3052,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3645,"record_offset":8484,"parent_record_offset":8420,"ancestor_record_offset":3052,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3645,"record_offset":8364,"parent_record_offset":8336,"ancestor_record_offset":3052,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3645,"record_offset":6040,"parent_record_offset":6008,"ancestor_record_offset":3052,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3645,"record_offset":6008,"parent_record_offset":5960,"ancestor_record_offset":3052,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3645,"record_offset":6576,"parent_record_offset":6544,"ancestor_record_offset":3052,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3645,"record_offset":5960,"parent_record_offset":5928,"ancestor_record_offset":3052,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3052,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":3052,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":3052,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5220,"parent_record_offset":5188,"ancestor_record_offset":3052,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5252,"parent_record_offset":5220,"ancestor_record_offset":3052,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5424,"parent_record_offset":5252,"ancestor_record_offset":3052,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3645,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":3052,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3645,"record_offset":6640,"parent_record_offset":6576,"ancestor_record_offset":3052,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3645,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":3052,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3645,"record_offset":6108,"parent_record_offset":6040,"ancestor_record_offset":3052,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3645,"record_offset":8752,"parent_record_offset":8724,"ancestor_record_offset":3052,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3645,"record_offset":3484,"parent_record_offset":3388,"ancestor_record_offset":3052,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3645,"record_offset":7580,"parent_record_offset":7548,"ancestor_record_offset":3052,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3645,"record_offset":7100,"parent_record_offset":7072,"ancestor_record_offset":3052,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3645,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":3052,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":3052,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":3052,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":4444,"parent_record_offset":3984,"ancestor_record_offset":3052,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":5160,"parent_record_offset":4980,"ancestor_record_offset":3052,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":6732,"parent_record_offset":6640,"ancestor_record_offset":3052,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":6172,"parent_record_offset":6040,"ancestor_record_offset":3052,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3645,"record_offset":6612,"parent_record_offset":6576,"ancestor_record_offset":3052,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3645,"record_offset":3796,"parent_record_offset":3768,"ancestor_record_offset":3052,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3645,"record_offset":5488,"parent_record_offset":5188,"ancestor_record_offset":3052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3645,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":3052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3645,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3052,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3645,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3052,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3645,"record_offset":4404,"parent_record_offset":4372,"ancestor_record_offset":3052,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3645,"record_offset":7268,"parent_record_offset":7240,"ancestor_record_offset":3052,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3645,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":3052,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3645,"record_offset":5888,"parent_record_offset":4940,"ancestor_record_offset":3052,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3645,"record_offset":3420,"parent_record_offset":3388,"ancestor_record_offset":3052,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3645,"record_offset":6384,"parent_record_offset":6040,"ancestor_record_offset":3052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3645,"record_offset":6800,"parent_record_offset":6772,"ancestor_record_offset":3052,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3645,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":3052,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3645,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":3052,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3645,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":3052,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3645,"record_offset":6220,"parent_record_offset":6172,"ancestor_record_offset":3052,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3645,"record_offset":6412,"parent_record_offset":6384,"ancestor_record_offset":3052,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3645,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3052,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3645,"record_offset":4500,"parent_record_offset":4476,"ancestor_record_offset":3052,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3645,"record_offset":4148,"parent_record_offset":4040,"ancestor_record_offset":3052,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3645,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":3052,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3645,"record_offset":3880,"parent_record_offset":3848,"ancestor_record_offset":3052,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3645,"record_offset":4012,"parent_record_offset":3984,"ancestor_record_offset":3052,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3645,"record_offset":3984,"parent_record_offset":3952,"ancestor_record_offset":3052,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3645,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3052,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3645,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3052,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3645,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3052,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3645,"record_offset":4476,"parent_record_offset":3952,"ancestor_record_offset":3052,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3584,"parent_record_offset":3552,"ancestor_record_offset":3052,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3744,"parent_record_offset":3584,"ancestor_record_offset":3052,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":3052,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":3052,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3052,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3360,"parent_record_offset":3328,"ancestor_record_offset":3052,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3645,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":3052,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3645,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":3052,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3645,"record_offset":4288,"parent_record_offset":4040,"ancestor_record_offset":3052,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3645,"record_offset":4260,"parent_record_offset":4040,"ancestor_record_offset":3052,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3645,"record_offset":4092,"parent_record_offset":4040,"ancestor_record_offset":3052,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":4900,"parent_record_offset":4860,"ancestor_record_offset":3052,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3645,"record_offset":5024,"parent_record_offset":4980,"ancestor_record_offset":3052,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":5188,"parent_record_offset":4980,"ancestor_record_offset":3052,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3645,"record_offset":5460,"parent_record_offset":5188,"ancestor_record_offset":3052,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3645,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":3052,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3645,"record_offset":5084,"parent_record_offset":5052,"ancestor_record_offset":3052,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3645,"record_offset":4980,"parent_record_offset":4940,"ancestor_record_offset":3052,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3645,"record_offset":4940,"parent_record_offset":4900,"ancestor_record_offset":3052,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3645,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":3052,"inlinee":7455,"source_line":398,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3645,"record_offset":8456,"parent_record_offset":8420,"ancestor_record_offset":3052,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/03645.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1469A
// 首条位置: {"module":2468,"inlinee":83610}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::impl$0::label
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":93396,"parent_record_offset":93372,"ancestor_record_offset":93372,"inlinee":17421,"source_line":47,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":93564,"parent_record_offset":93396,"ancestor_record_offset":93372,"inlinee":8030,"source_line":859,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":93436,"parent_record_offset":93396,"ancestor_record_offset":93372,"inlinee":17363,"source_line":66,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":93472,"parent_record_offset":93436,"ancestor_record_offset":93372,"inlinee":5596,"source_line":96,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2468,"record_offset":93500,"parent_record_offset":93472,"ancestor_record_offset":93372,"inlinee":5598,"source_line":129,"archive":"_pdb/A/modules/02468.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$0::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2468,"record_offset":93532,"parent_record_offset":93436,"ancestor_record_offset":93372,"inlinee":17365,"source_line":49,"archive":"_pdb/A/modules/02468.jsonl.gz"}
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
// 完整结构及原始类型记录: demo_show_max_elevation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: demo_show_max_elevation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: demo_show_max_elevation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 信息 ID: 0913a28d1b6c4ff7785aec20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 35157dae68b0e0e2917df39f: void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> > @ 0x15e2213 -> 0x1b5b6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> @ 0x165d84a -> 0x1b5b6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 信息 ID: a8017cb8cab2463e93136723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation", "captures": "unknown"}
// [闭包环境类型引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=2468, inline_record=93372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
