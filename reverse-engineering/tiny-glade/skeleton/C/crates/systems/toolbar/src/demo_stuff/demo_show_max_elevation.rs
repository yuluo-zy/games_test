// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/toolbar/src/demo_stuff/demo_show_max_elevation.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 2d10a437cafefd948c6905889250e31256721339992ec57ba89296cd8797c663
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: demo_show_max_elevation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1239, 4205
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4205,"record_offset":2080,"rva":43437872,"va":5412146992}
// 编译实例 RVAs: 0x296cf30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-36469445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e83b85d3fd","size":5,"soft_fail":false,"target_va":5375678688,"va":5412148128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-36470286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f281d3fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5412148985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43425926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e87a5f69fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5412148113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42909277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8a34171fd","size":5,"soft_fail":false,"target_va":5369240528,"va":5412149800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31084560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f0af25fe","size":5,"soft_fail":false,"target_va":5381064176,"va":5412148731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28822758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e81a3348fe","size":5,"soft_fail":false,"target_va":5383325056,"va":5412147809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25102955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e895f580fe","size":5,"soft_fail":false,"target_va":5387045984,"va":5412148934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141196180
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24999266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e89e8a82fe","size":5,"soft_fail":false,"target_va":5387149696,"va":5412148957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22361926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8bac8aafe","size":5,"soft_fail":false,"target_va":5389786032,"va":5412147953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: demo_show_max_elevation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4202601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e869204000","size":5,"soft_fail":false,"target_va":5416351392,"va":5412148786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4471566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e80e3b4400","size":5,"soft_fail":false,"target_va":5416620769,"va":5412149198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4472241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8b13d4400","size":5,"soft_fail":false,"target_va":5416621408,"va":5412149162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4472472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8983e4400","size":5,"soft_fail":false,"target_va":5416621762,"va":5412149285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e83bfbffff","size":5,"soft_fail":false,"target_va":5412145936,"va":5412147152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4472048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f03c4400","size":5,"soft_fail":false,"target_va":5416621376,"va":5412149323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30539535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f1002efe","size":5,"soft_fail":false,"target_va":5381607568,"va":5412147098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::as_vec2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30391743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8414230fe","size":5,"soft_fail":false,"target_va":5381756080,"va":5412147818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32550322","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e84e520ffe","size":5,"soft_fail":false,"target_va":5379597392,"va":5412147709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32469257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f78e10fe","size":5,"soft_fail":false,"target_va":5379678976,"va":5412148228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32469187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e83d8f10fe","size":5,"soft_fail":false,"target_va":5379679024,"va":5412148206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32469161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8578f10fe","size":5,"soft_fail":false,"target_va":5379679088,"va":5412148244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::world_space_to_screen_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33109473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e81fca06fe","size":5,"soft_fail":false,"target_va":5379038368,"va":5412147836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28183710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e862f351fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5412147497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28175203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e89d1452fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5412147422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28184166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e89af151fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5412149073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28184300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e814f151fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5412147111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_right
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-28735122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e86e8949fe","size":5,"soft_fail":false,"target_va":5383412768,"va":5412147885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4507383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8f7c64400","size":5,"soft_fail":false,"target_va":5416656560,"va":5412149172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-24818589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e8634c85fe","size":5,"soft_fail":false,"target_va":5387331072,"va":5412149656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4470105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e859354400","size":5,"soft_fail":false,"target_va":5416619360,"va":5412149250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-35942396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412146992,"raw_hex":"e80490dbfd","size":5,"soft_fail":false,"target_va":5376206368,"va":5412148759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4205,"record_offset":6376,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4205,"record_offset":6192,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":5812,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":5908,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4205,"record_offset":7464,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4205,"record_offset":5540,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":13153,"source_line":26,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":4205,"record_offset":4512,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":7872,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4205,"record_offset":6020,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4205,"record_offset":6076,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4205,"record_offset":6644,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4205,"record_offset":6104,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4205,"record_offset":6884,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":3704,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":2304,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":2692,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$20::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":4205,"record_offset":7844,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21729,"source_line":167,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4205,"record_offset":7524,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21728,"source_line":488,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":2080,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":2080,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":2080,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":2080,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2080,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":2080,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":6252,"parent_record_offset":6220,"ancestor_record_offset":2080,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":4205,"record_offset":8056,"parent_record_offset":7968,"ancestor_record_offset":2080,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":4205,"record_offset":6220,"parent_record_offset":6192,"ancestor_record_offset":2080,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":2080,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":7996,"parent_record_offset":7968,"ancestor_record_offset":2080,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":2080,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":2080,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":2080,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4205,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":2080,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4205,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":2080,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":7680,"parent_record_offset":7640,"ancestor_record_offset":2080,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":2080,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":4800,"parent_record_offset":4772,"ancestor_record_offset":2080,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":2080,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":7616,"parent_record_offset":7548,"ancestor_record_offset":2080,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":2080,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4205,"record_offset":4772,"parent_record_offset":4648,"ancestor_record_offset":2080,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4205,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":2080,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4205,"record_offset":4648,"parent_record_offset":4608,"ancestor_record_offset":2080,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":4205,"record_offset":5580,"parent_record_offset":5540,"ancestor_record_offset":2080,"inlinee":10306,"source_line":112,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":4205,"record_offset":4608,"parent_record_offset":4552,"ancestor_record_offset":2080,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":4205,"record_offset":4552,"parent_record_offset":4512,"ancestor_record_offset":2080,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":2080,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":4060,"parent_record_offset":4032,"ancestor_record_offset":2080,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":2080,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":2080,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":4120,"parent_record_offset":4004,"ancestor_record_offset":2080,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4205,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":2080,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":4205,"record_offset":5624,"parent_record_offset":5580,"ancestor_record_offset":2080,"inlinee":10307,"source_line":174,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4205,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":2080,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4205,"record_offset":5208,"parent_record_offset":5180,"ancestor_record_offset":2080,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4205,"record_offset":4716,"parent_record_offset":4648,"ancestor_record_offset":2080,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":4205,"record_offset":8024,"parent_record_offset":7996,"ancestor_record_offset":2080,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4205,"record_offset":6132,"parent_record_offset":6104,"ancestor_record_offset":2080,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4205,"record_offset":6676,"parent_record_offset":6644,"ancestor_record_offset":2080,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4205,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":2080,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4205,"record_offset":3852,"parent_record_offset":3812,"ancestor_record_offset":2080,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4205,"record_offset":5672,"parent_record_offset":5624,"ancestor_record_offset":2080,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4205,"record_offset":5744,"parent_record_offset":5624,"ancestor_record_offset":2080,"inlinee":10308,"source_line":1621,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4205,"record_offset":5776,"parent_record_offset":5580,"ancestor_record_offset":2080,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4205,"record_offset":2640,"parent_record_offset":2612,"ancestor_record_offset":2080,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4205,"record_offset":4468,"parent_record_offset":3812,"ancestor_record_offset":2080,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":4184,"parent_record_offset":3952,"ancestor_record_offset":2080,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4205,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":2080,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4205,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2080,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4205,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":2080,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4205,"record_offset":3296,"parent_record_offset":3268,"ancestor_record_offset":2080,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4205,"record_offset":7800,"parent_record_offset":7640,"ancestor_record_offset":2080,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4205,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":2080,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4205,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2080,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4205,"record_offset":5124,"parent_record_offset":4648,"ancestor_record_offset":2080,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4205,"record_offset":5380,"parent_record_offset":4648,"ancestor_record_offset":2080,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4205,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":2080,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4205,"record_offset":5868,"parent_record_offset":5840,"ancestor_record_offset":2080,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4205,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":2080,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4205,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":2080,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4205,"record_offset":5408,"parent_record_offset":5380,"ancestor_record_offset":2080,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4205,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":2080,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4205,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":2080,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4205,"record_offset":3424,"parent_record_offset":3396,"ancestor_record_offset":2080,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4205,"record_offset":3016,"parent_record_offset":2900,"ancestor_record_offset":2080,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4205,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":2080,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4205,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":2080,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4205,"record_offset":2868,"parent_record_offset":2836,"ancestor_record_offset":2080,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4205,"record_offset":2836,"parent_record_offset":2800,"ancestor_record_offset":2080,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4205,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2080,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4205,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2080,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4205,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2080,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4205,"record_offset":3340,"parent_record_offset":2800,"ancestor_record_offset":2080,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":2080,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":2080,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2532,"parent_record_offset":2500,"ancestor_record_offset":2080,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2588,"parent_record_offset":2532,"ancestor_record_offset":2080,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":2080,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2348,"parent_record_offset":2304,"ancestor_record_offset":2080,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4205,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2080,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4205,"record_offset":2900,"parent_record_offset":2868,"ancestor_record_offset":2080,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4205,"record_offset":3184,"parent_record_offset":2900,"ancestor_record_offset":2080,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4205,"record_offset":3156,"parent_record_offset":2900,"ancestor_record_offset":2080,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4205,"record_offset":2960,"parent_record_offset":2900,"ancestor_record_offset":2080,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":3740,"parent_record_offset":3704,"ancestor_record_offset":2080,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":8544,"parent_record_offset":8512,"ancestor_record_offset":2080,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":8996,"parent_record_offset":8792,"ancestor_record_offset":2080,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4205,"record_offset":3880,"parent_record_offset":3852,"ancestor_record_offset":2080,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":3952,"parent_record_offset":3812,"ancestor_record_offset":2080,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4205,"record_offset":4156,"parent_record_offset":3952,"ancestor_record_offset":2080,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4205,"record_offset":3908,"parent_record_offset":3880,"ancestor_record_offset":2080,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4205,"record_offset":3812,"parent_record_offset":3776,"ancestor_record_offset":2080,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4205,"record_offset":3776,"parent_record_offset":3740,"ancestor_record_offset":2080,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4205,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":2080,"inlinee":10221,"source_line":526,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":4205,"record_offset":7588,"parent_record_offset":7548,"ancestor_record_offset":2080,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/04205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1122A
// 首条位置: {"module":1239,"inlinee":70186}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":112952,"parent_record_offset":112832,"ancestor_record_offset":112832,"inlinee":9902,"source_line":273,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":113012,"parent_record_offset":112832,"ancestor_record_offset":112832,"inlinee":21815,"source_line":295,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$27::label
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":112428,"parent_record_offset":112404,"ancestor_record_offset":112404,"inlinee":21730,"source_line":43,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":112536,"parent_record_offset":112480,"ancestor_record_offset":112404,"inlinee":8058,"source_line":1443,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":112480,"parent_record_offset":112428,"ancestor_record_offset":112404,"inlinee":10209,"source_line":1427,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":112452,"parent_record_offset":112428,"ancestor_record_offset":112404,"inlinee":21696,"source_line":85,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":112508,"parent_record_offset":112480,"ancestor_record_offset":112404,"inlinee":8062,"source_line":115,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1239,"record_offset":112560,"parent_record_offset":112536,"ancestor_record_offset":112404,"inlinee":8064,"source_line":148,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":112980,"parent_record_offset":112952,"ancestor_record_offset":112832,"inlinee":21733,"source_line":234,"archive":"_pdb/C/modules/01239.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$27::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1239,"record_offset":112916,"parent_record_offset":112888,"ancestor_record_offset":112832,"inlinee":21732,"source_line":45,"archive":"_pdb/C/modules/01239.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation
// 信息 ID: 24e290d4b6e7da4b5895ac86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4646b406fad6ed9ad2d39eae: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> > @ 0x230d2ea -> 0x296cf30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::resources::glade_border::MaxDemoElevationReached>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)> @ 0x23b2535 -> 0x296cf30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure$0
// 信息 ID: 3dc546772c1619b260fa323d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation", "captures": "unknown"}
// [闭包环境类型引用] toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1239, inline_record=112404; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1239, inline_record=112632; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::demo_stuff::demo_show_max_elevation::demo_show_max_elevation::closure_env$0,ref$<str$> >; module=1239, inline_record=112832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
