// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/demo_stuff/demo_show_max_elevation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: cf00da7362f72cda3b66c744ce0d136c1bb8be37637d38018075fed3553f3cd1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: demo_show_max_elevation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1205, 3914
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3914,"record_offset":2036,"rva":37004304,"va":5405713424}
// 编译实例 RVAs: 0x234a410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30985671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8393227fe","size":5,"soft_fail":false,"target_va":5374729632,"va":5405715298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30902861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8b37528fe","size":5,"soft_fail":false,"target_va":5374813424,"va":5405716280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30633220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8fc922cfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5405714703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30634054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8ba8f2cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405715553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23779830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e80a2695fe","size":5,"soft_fail":false,"target_va":5381934560,"va":5405714385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20628165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e83b3dc5fe","size":5,"soft_fail":false,"target_va":5385087344,"va":5405715504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbed00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20496203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8b540c7fe","size":5,"soft_fail":false,"target_va":5385219328,"va":5405715526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141246fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17840307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e84dc7effe","size":5,"soft_fail":false,"target_va":5387874224,"va":5405714526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5825149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e87de25800","size":5,"soft_fail":false,"target_va":5411540992,"va":5405715838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30511571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e82d6e2efe","size":5,"soft_fail":false,"target_va":5375203792,"va":5405715358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5825774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8eee45800","size":5,"soft_fail":false,"target_va":5411541584,"va":5405715805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5826470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8a6e75800","size":5,"soft_fail":false,"target_va":5411542296,"va":5405715821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e82ffbffff","size":5,"soft_fail":false,"target_va":5405712352,"va":5405713580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5829698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e842f45800","size":5,"soft_fail":false,"target_va":5411543360,"va":5405713657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27306175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e841575ffe","size":5,"soft_fail":false,"target_va":5378407360,"va":5405713530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27265998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e832f45ffe","size":5,"soft_fail":false,"target_va":5378448400,"va":5405714393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18134222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8324bebfe","size":5,"soft_fail":false,"target_va":5387580080,"va":5405714297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25451063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8c9a57bfe","size":5,"soft_fail":false,"target_va":5380263744,"va":5405714802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25451250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e80ea57bfe","size":5,"soft_fail":false,"target_va":5380263536,"va":5405714781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25451030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8eaa57bfe","size":5,"soft_fail":false,"target_va":5380263792,"va":5405714817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25698415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e891df77fe","size":5,"soft_fail":false,"target_va":5380016000,"va":5405714410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24328555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e895c68cfe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405714086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24323366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8dada8cfe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405714001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24329180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e824c48cfe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405715655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24328844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e874c58cfe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405713543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5879084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e82cb55900","size":5,"soft_fail":false,"target_va":5411594832,"va":5405715743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20427405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8734dc8fe","size":5,"soft_fail":false,"target_va":5385288736,"va":5405716136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5821306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e87ad35800","size":5,"soft_fail":false,"target_va":5411537104,"va":5405715793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30174402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e83e9333fe","size":5,"soft_fail":false,"target_va":5375540928,"va":5405715325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_right
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23804455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405713424,"raw_hex":"e8d9c594fe","size":5,"soft_fail":false,"target_va":5381910000,"va":5405714450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":6360,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3914,"record_offset":6512,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":5976,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":6072,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3914,"record_offset":7536,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3914,"record_offset":5704,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3914,"record_offset":4676,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3914,"record_offset":7916,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3914,"record_offset":6184,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3914,"record_offset":6244,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3914,"record_offset":6748,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3914,"record_offset":6272,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3914,"record_offset":6988,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":3692,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":2260,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":2716,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":3914,"record_offset":7888,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3914,"record_offset":7596,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":61863,"source_line":456,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":6416,"parent_record_offset":6392,"ancestor_record_offset":2036,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":6440,"parent_record_offset":6416,"ancestor_record_offset":2036,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":2036,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":6392,"parent_record_offset":6360,"ancestor_record_offset":2036,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3914,"record_offset":8100,"parent_record_offset":8012,"ancestor_record_offset":2036,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":2036,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":8040,"parent_record_offset":8012,"ancestor_record_offset":2036,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":8012,"parent_record_offset":7988,"ancestor_record_offset":2036,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":5064,"parent_record_offset":5040,"ancestor_record_offset":2036,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":2036,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3914,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":2036,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3914,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":2036,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":7728,"parent_record_offset":7688,"ancestor_record_offset":2036,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":2036,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":2036,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":2036,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":7688,"parent_record_offset":7620,"ancestor_record_offset":2036,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3914,"record_offset":4936,"parent_record_offset":4812,"ancestor_record_offset":2036,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3914,"record_offset":7564,"parent_record_offset":7536,"ancestor_record_offset":2036,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3914,"record_offset":4772,"parent_record_offset":4716,"ancestor_record_offset":2036,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3914,"record_offset":4812,"parent_record_offset":4772,"ancestor_record_offset":2036,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3914,"record_offset":5744,"parent_record_offset":5704,"ancestor_record_offset":2036,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3914,"record_offset":4716,"parent_record_offset":4676,"ancestor_record_offset":2036,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4028,"parent_record_offset":4000,"ancestor_record_offset":2036,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":2036,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4056,"parent_record_offset":4028,"ancestor_record_offset":2036,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":3976,"parent_record_offset":3948,"ancestor_record_offset":2036,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":2036,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4172,"parent_record_offset":4000,"ancestor_record_offset":2036,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3914,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":2036,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3914,"record_offset":5788,"parent_record_offset":5744,"ancestor_record_offset":2036,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3914,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":2036,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3914,"record_offset":5372,"parent_record_offset":5344,"ancestor_record_offset":2036,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3914,"record_offset":4880,"parent_record_offset":4812,"ancestor_record_offset":2036,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3914,"record_offset":8068,"parent_record_offset":8040,"ancestor_record_offset":2036,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3914,"record_offset":2416,"parent_record_offset":2384,"ancestor_record_offset":2036,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3914,"record_offset":6300,"parent_record_offset":6272,"ancestor_record_offset":2036,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3914,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":2036,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3914,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":2036,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3914,"record_offset":3844,"parent_record_offset":3800,"ancestor_record_offset":2036,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3914,"record_offset":5836,"parent_record_offset":5788,"ancestor_record_offset":2036,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3914,"record_offset":5908,"parent_record_offset":5788,"ancestor_record_offset":2036,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3914,"record_offset":5940,"parent_record_offset":5744,"ancestor_record_offset":2036,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3914,"record_offset":2664,"parent_record_offset":2636,"ancestor_record_offset":2036,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3914,"record_offset":4632,"parent_record_offset":3800,"ancestor_record_offset":2036,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3914,"record_offset":4236,"parent_record_offset":3948,"ancestor_record_offset":2036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3914,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2036,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3914,"record_offset":3128,"parent_record_offset":3104,"ancestor_record_offset":2036,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3914,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":2036,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3914,"record_offset":3312,"parent_record_offset":3284,"ancestor_record_offset":2036,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3914,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":2036,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3914,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":2036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3914,"record_offset":5288,"parent_record_offset":4812,"ancestor_record_offset":2036,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3914,"record_offset":5544,"parent_record_offset":4812,"ancestor_record_offset":2036,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3914,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":2036,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3914,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":2036,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3914,"record_offset":5344,"parent_record_offset":5316,"ancestor_record_offset":2036,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3914,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":2036,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3914,"record_offset":5572,"parent_record_offset":5544,"ancestor_record_offset":2036,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3914,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":2036,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3914,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":2036,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3914,"record_offset":3384,"parent_record_offset":3356,"ancestor_record_offset":2036,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3914,"record_offset":3032,"parent_record_offset":2916,"ancestor_record_offset":2036,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3914,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":2036,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3914,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":2036,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3914,"record_offset":2884,"parent_record_offset":2852,"ancestor_record_offset":2036,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3914,"record_offset":2852,"parent_record_offset":2816,"ancestor_record_offset":2036,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3914,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2036,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3914,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":2036,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3914,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2036,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3914,"record_offset":3356,"parent_record_offset":2816,"ancestor_record_offset":2036,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2356,"parent_record_offset":2324,"ancestor_record_offset":2036,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2324,"parent_record_offset":2296,"ancestor_record_offset":2036,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":2036,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2612,"parent_record_offset":2516,"ancestor_record_offset":2036,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":2036,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2296,"parent_record_offset":2260,"ancestor_record_offset":2036,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3914,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2036,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3914,"record_offset":2916,"parent_record_offset":2884,"ancestor_record_offset":2036,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3914,"record_offset":3200,"parent_record_offset":2916,"ancestor_record_offset":2036,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3914,"record_offset":3172,"parent_record_offset":2916,"ancestor_record_offset":2036,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3914,"record_offset":2976,"parent_record_offset":2916,"ancestor_record_offset":2036,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":3728,"parent_record_offset":3692,"ancestor_record_offset":2036,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":8476,"parent_record_offset":8444,"ancestor_record_offset":2036,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":8976,"parent_record_offset":8716,"ancestor_record_offset":2036,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3914,"record_offset":3872,"parent_record_offset":3844,"ancestor_record_offset":2036,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":3948,"parent_record_offset":3800,"ancestor_record_offset":2036,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3914,"record_offset":4208,"parent_record_offset":3948,"ancestor_record_offset":2036,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3914,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":2036,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3914,"record_offset":3800,"parent_record_offset":3764,"ancestor_record_offset":2036,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3914,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":2036,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3914,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":2036,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3914,"record_offset":7660,"parent_record_offset":7620,"ancestor_record_offset":2036,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/03914.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF27B
// 首条位置: {"module":1205,"inlinee":62075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":103200,"parent_record_offset":103176,"ancestor_record_offset":103176,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103304,"parent_record_offset":103252,"ancestor_record_offset":103176,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103252,"parent_record_offset":103200,"ancestor_record_offset":103176,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103772,"parent_record_offset":103748,"ancestor_record_offset":103596,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103224,"parent_record_offset":103200,"ancestor_record_offset":103176,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103276,"parent_record_offset":103252,"ancestor_record_offset":103176,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":103328,"parent_record_offset":103304,"ancestor_record_offset":103176,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":103748,"parent_record_offset":103620,"ancestor_record_offset":103596,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":103716,"parent_record_offset":103656,"ancestor_record_offset":103596,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 信息 ID: 29ad9c09ca572b02b9edf6e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 41b778c0a12a9e7d44002302: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> > > @ 0x1e0fd3b -> 0x234a410 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> > @ 0x1eae059 -> 0x234a410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 信息 ID: 9802b699d73a9f3a981c3c7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation", "captures": "unknown"}
// [闭包环境类型引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1205, inline_record=103176; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1205, inline_record=103400; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1205, inline_record=103596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
