// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c9f397ae300c649fb255c635ecde0bef81d189c1cb8944b8be62c869dd7d89f2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sample_wall_path_intersections.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 389
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":389,"record_offset":1660,"rva":7653152,"va":5376362272}
// 编译实例 RVAs: 0x74c720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1686974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e84242e6ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376363305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1283965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e88368ecff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376365448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1287967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8e158ecff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376369466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-847490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e87e11f3ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5376363517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=20
//     target_aliases=2 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2916474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e87a802c00","size":5,"soft_fail":false,"target_va":5379280368,"va":5376363889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fa190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11189888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e880beaa00","size":5,"soft_fail":false,"target_va":5387559312,"va":5376369419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35483523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8836f1d02","size":5,"soft_fail":false,"target_va":5411847200,"va":5376363672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ec0
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: sample_wall_path_intersections.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35483636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8f46f1d02","size":5,"soft_fail":false,"target_va":5411847872,"va":5376364231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t4529025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8811b4500","size":5,"soft_fail":false,"target_va":5380895104,"va":5376366074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34969839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8ef981502","size":5,"soft_fail":false,"target_va":5411339744,"va":5376369900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35170495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8bfa81802","size":5,"soft_fail":false,"target_va":5411540963,"va":5376370463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35171267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8c3ab1802","size":5,"soft_fail":false,"target_va":5411541584,"va":5376370312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35171757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8adad1802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376370014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35171964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e87cae1802","size":5,"soft_fail":false,"target_va":5411542296,"va":5376370327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35171515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8bbac1802","size":5,"soft_fail":false,"target_va":5411541904,"va":5376370384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e830fbffff","size":5,"soft_fail":false,"target_va":5376361200,"va":5376362427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35180851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e833d11802","size":5,"soft_fail":false,"target_va":5411543360,"va":5376362504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3768906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e84a823900","size":5,"soft_fail":false,"target_va":5380131872,"va":5376362961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5022611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e893a34c00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376362920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5027800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8d8b74c00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376362835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5016345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8198b4c00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376370130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5022302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e85ea24c00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376362397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35270513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e8712f1a02","size":5,"soft_fail":false,"target_va":5411640768,"va":5376370250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35166799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376362272,"raw_hex":"e84f9a1802","size":5,"soft_fail":false,"target_va":5411537104,"va":5376370300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":8332,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":4420,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":6568,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5294,"source_line":3002,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":9444,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":8360,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":389,"record_offset":4360,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":389,"record_offset":5148,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":389,"record_offset":4448,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":389,"record_offset":8304,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":389,"record_offset":20316,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":389,"record_offset":9336,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7484,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":389,"record_offset":7800,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":389,"record_offset":7168,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":18996,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":19024,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":389,"record_offset":9552,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":389,"record_offset":7724,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24398,"source_line":108,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":7924,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":7396,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":389,"record_offset":7308,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":3376,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":1952,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":2400,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":389,"record_offset":5528,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":7896,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":9696,"parent_record_offset":1660,"ancestor_record_offset":1660,"inlinee":24400,"source_line":135,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":1660,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10324,"parent_record_offset":10300,"ancestor_record_offset":1660,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":1660,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10276,"parent_record_offset":10092,"ancestor_record_offset":1660,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":17740,"parent_record_offset":17648,"ancestor_record_offset":1660,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6896,"parent_record_offset":6852,"ancestor_record_offset":1660,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6716,"parent_record_offset":6688,"ancestor_record_offset":1660,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10020,"parent_record_offset":9984,"ancestor_record_offset":1660,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":1660,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17676,"parent_record_offset":17648,"ancestor_record_offset":1660,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17648,"parent_record_offset":17624,"ancestor_record_offset":1660,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":1660,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":1660,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":1660,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":6744,"parent_record_offset":6716,"ancestor_record_offset":1660,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10092,"parent_record_offset":10056,"ancestor_record_offset":1660,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10056,"parent_record_offset":10020,"ancestor_record_offset":1660,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17624,"parent_record_offset":17600,"ancestor_record_offset":1660,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10136,"parent_record_offset":10092,"ancestor_record_offset":1660,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":6852,"parent_record_offset":6632,"ancestor_record_offset":1660,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":1660,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":4792,"parent_record_offset":4768,"ancestor_record_offset":1660,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":8424,"parent_record_offset":8392,"ancestor_record_offset":1660,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":6632,"parent_record_offset":6568,"ancestor_record_offset":1660,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":6604,"parent_record_offset":6568,"ancestor_record_offset":1660,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":10940,"parent_record_offset":10916,"ancestor_record_offset":1660,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":6688,"parent_record_offset":6632,"ancestor_record_offset":1660,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":9948,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":9984,"parent_record_offset":9948,"ancestor_record_offset":1660,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17112,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":8852,"parent_record_offset":8808,"ancestor_record_offset":1660,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":4768,"parent_record_offset":4536,"ancestor_record_offset":1660,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":1660,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":389,"record_offset":7132,"parent_record_offset":6632,"ancestor_record_offset":1660,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":389,"record_offset":9296,"parent_record_offset":8424,"ancestor_record_offset":1660,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":389,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":1660,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":389,"record_offset":8392,"parent_record_offset":8360,"ancestor_record_offset":1660,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":389,"record_offset":4388,"parent_record_offset":4360,"ancestor_record_offset":1660,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":389,"record_offset":4512,"parent_record_offset":4472,"ancestor_record_offset":1660,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":389,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":1660,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":389,"record_offset":5180,"parent_record_offset":5148,"ancestor_record_offset":1660,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":389,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":1660,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":1660,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":1660,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":1660,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":1660,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":1660,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3660,"parent_record_offset":3632,"ancestor_record_offset":1660,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":1660,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3856,"parent_record_offset":3684,"ancestor_record_offset":1660,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":389,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":1660,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":1660,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":1660,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":1660,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":1660,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":1660,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":389,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":1660,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":389,"record_offset":5916,"parent_record_offset":5892,"ancestor_record_offset":1660,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":389,"record_offset":11660,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":389,"record_offset":7764,"parent_record_offset":7724,"ancestor_record_offset":1660,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":389,"record_offset":5856,"parent_record_offset":5832,"ancestor_record_offset":1660,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":389,"record_offset":6392,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":389,"record_offset":5256,"parent_record_offset":5224,"ancestor_record_offset":1660,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":389,"record_offset":5224,"parent_record_offset":5180,"ancestor_record_offset":1660,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":389,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":1660,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":389,"record_offset":8764,"parent_record_offset":8664,"ancestor_record_offset":1660,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":389,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":1660,"inlinee":11460,"source_line":95,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":389,"record_offset":8632,"parent_record_offset":8600,"ancestor_record_offset":1660,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":389,"record_offset":7548,"parent_record_offset":7512,"ancestor_record_offset":1660,"inlinee":11714,"source_line":132,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":389,"record_offset":11116,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":1660,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":1660,"inlinee":9837,"source_line":637,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11216,"parent_record_offset":11180,"ancestor_record_offset":1660,"inlinee":24402,"source_line":108,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11148,"parent_record_offset":11116,"ancestor_record_offset":1660,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":1660,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":1660,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7512,"parent_record_offset":7484,"ancestor_record_offset":1660,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":1660,"inlinee":9835,"source_line":614,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":389,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":1660,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":389,"record_offset":6984,"parent_record_offset":6632,"ancestor_record_offset":1660,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":389,"record_offset":8600,"parent_record_offset":8424,"ancestor_record_offset":1660,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":389,"record_offset":8808,"parent_record_offset":8764,"ancestor_record_offset":1660,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":389,"record_offset":10636,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":389,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":1660,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":1660,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":1660,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":1660,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":4584,"parent_record_offset":4536,"ancestor_record_offset":1660,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":9000,"parent_record_offset":8664,"ancestor_record_offset":1660,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":17708,"parent_record_offset":17676,"ancestor_record_offset":1660,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":389,"record_offset":2100,"parent_record_offset":2068,"ancestor_record_offset":1660,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":389,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":1660,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":389,"record_offset":6036,"parent_record_offset":6008,"ancestor_record_offset":1660,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":7424,"parent_record_offset":7396,"ancestor_record_offset":1660,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":7952,"parent_record_offset":7924,"ancestor_record_offset":1660,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":7336,"parent_record_offset":7308,"ancestor_record_offset":1660,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":389,"record_offset":3528,"parent_record_offset":3484,"ancestor_record_offset":1660,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":1660,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":5320,"parent_record_offset":5224,"ancestor_record_offset":1660,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":1660,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":389,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":1660,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":389,"record_offset":4316,"parent_record_offset":3484,"ancestor_record_offset":1660,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":3920,"parent_record_offset":3632,"ancestor_record_offset":1660,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":389,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":1660,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":389,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":1660,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":5468,"parent_record_offset":5444,"ancestor_record_offset":1660,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":2812,"parent_record_offset":2788,"ancestor_record_offset":1660,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":1660,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":2996,"parent_record_offset":2968,"ancestor_record_offset":1660,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":1660,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":389,"record_offset":4640,"parent_record_offset":4536,"ancestor_record_offset":1660,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":389,"record_offset":4992,"parent_record_offset":4536,"ancestor_record_offset":1660,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":9476,"parent_record_offset":9444,"ancestor_record_offset":1660,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":9580,"parent_record_offset":9552,"ancestor_record_offset":1660,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":9508,"parent_record_offset":9476,"ancestor_record_offset":1660,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":4696,"parent_record_offset":4668,"ancestor_record_offset":1660,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":389,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":1660,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":389,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":1660,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":1660,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":5444,"parent_record_offset":5420,"ancestor_record_offset":1660,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":1660,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":17424,"parent_record_offset":17400,"ancestor_record_offset":1660,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":1660,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":3068,"parent_record_offset":3040,"ancestor_record_offset":1660,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":8664,"parent_record_offset":8632,"ancestor_record_offset":1660,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":5396,"parent_record_offset":5344,"ancestor_record_offset":1660,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":2716,"parent_record_offset":2600,"ancestor_record_offset":1660,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":1660,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":9364,"parent_record_offset":9336,"ancestor_record_offset":1660,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":17400,"parent_record_offset":17308,"ancestor_record_offset":1660,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":389,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":1660,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":389,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":1660,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":389,"record_offset":2568,"parent_record_offset":2536,"ancestor_record_offset":1660,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":389,"record_offset":2536,"parent_record_offset":2500,"ancestor_record_offset":1660,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":389,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1660,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":389,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1660,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":389,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1660,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":389,"record_offset":3040,"parent_record_offset":2500,"ancestor_record_offset":1660,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2040,"parent_record_offset":2008,"ancestor_record_offset":1660,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":1660,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1660,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2296,"parent_record_offset":2200,"ancestor_record_offset":1660,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1660,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":1984,"parent_record_offset":1952,"ancestor_record_offset":1660,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":389,"record_offset":2320,"parent_record_offset":2296,"ancestor_record_offset":1660,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":389,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":1660,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":389,"record_offset":2884,"parent_record_offset":2600,"ancestor_record_offset":1660,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":389,"record_offset":2856,"parent_record_offset":2600,"ancestor_record_offset":1660,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":389,"record_offset":2660,"parent_record_offset":2600,"ancestor_record_offset":1660,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":389,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":1660,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":389,"record_offset":14020,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":24398,"source_line":108,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13568,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13680,"parent_record_offset":13592,"ancestor_record_offset":1660,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11688,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11396,"parent_record_offset":11368,"ancestor_record_offset":1660,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":1660,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11860,"parent_record_offset":11688,"ancestor_record_offset":1660,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":1660,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":389,"record_offset":5976,"parent_record_offset":5948,"ancestor_record_offset":1660,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":389,"record_offset":6008,"parent_record_offset":5976,"ancestor_record_offset":1660,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":389,"record_offset":5948,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":389,"record_offset":5808,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":389,"record_offset":6420,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":389,"record_offset":5892,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":389,"record_offset":5780,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":389,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":1660,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":389,"record_offset":5584,"parent_record_offset":5556,"ancestor_record_offset":1660,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":389,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":1660,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":389,"record_offset":6276,"parent_record_offset":6108,"ancestor_record_offset":1660,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":389,"record_offset":5620,"parent_record_offset":5584,"ancestor_record_offset":1660,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":6216,"parent_record_offset":6132,"ancestor_record_offset":1660,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":1660,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":6512,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":1660,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":1660,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":5648,"parent_record_offset":5584,"ancestor_record_offset":1660,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":1660,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":5752,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":389,"record_offset":6108,"parent_record_offset":5696,"ancestor_record_offset":1660,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":3412,"parent_record_offset":3376,"ancestor_record_offset":1660,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":19324,"parent_record_offset":19292,"ancestor_record_offset":1660,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":19840,"parent_record_offset":19580,"ancestor_record_offset":1660,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":389,"record_offset":3556,"parent_record_offset":3528,"ancestor_record_offset":1660,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":3632,"parent_record_offset":3484,"ancestor_record_offset":1660,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":389,"record_offset":3892,"parent_record_offset":3632,"ancestor_record_offset":1660,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":17516,"parent_record_offset":17488,"ancestor_record_offset":1660,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":13712,"parent_record_offset":13680,"ancestor_record_offset":1660,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":389,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":1660,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":389,"record_offset":3484,"parent_record_offset":3448,"ancestor_record_offset":1660,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":389,"record_offset":3448,"parent_record_offset":3412,"ancestor_record_offset":1660,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":10916,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":24401,"source_line":268,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":17488,"parent_record_offset":9696,"ancestor_record_offset":1660,"inlinee":24404,"source_line":285,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":7684,"parent_record_offset":7548,"ancestor_record_offset":1660,"inlinee":24396,"source_line":57,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F4C
// 首条位置: {"module":389,"inlinee":24396}
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
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F50
// 首条位置: {"module":389,"inlinee":24400}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":9948,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17112,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":389,"record_offset":11660,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":389,"record_offset":11116,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":389,"record_offset":10636,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":17552,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":17308,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":389,"record_offset":14020,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":24398,"source_line":108,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13568,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11688,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":10916,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":24401,"source_line":268,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/sample_wall_path_intersections.rs
//     evidence={"module":389,"record_offset":17488,"parent_record_offset":9696,"ancestor_record_offset":9696,"inlinee":24404,"source_line":285,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":9696,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10324,"parent_record_offset":10300,"ancestor_record_offset":9696,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":17764,"parent_record_offset":17740,"ancestor_record_offset":9696,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":10276,"parent_record_offset":10092,"ancestor_record_offset":9696,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":17740,"parent_record_offset":17648,"ancestor_record_offset":9696,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":9696,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10020,"parent_record_offset":9984,"ancestor_record_offset":9696,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17676,"parent_record_offset":17648,"ancestor_record_offset":9696,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17648,"parent_record_offset":17624,"ancestor_record_offset":9696,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":9696,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":9696,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10092,"parent_record_offset":10056,"ancestor_record_offset":9696,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10056,"parent_record_offset":10020,"ancestor_record_offset":9696,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":17624,"parent_record_offset":17600,"ancestor_record_offset":9696,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10136,"parent_record_offset":10092,"ancestor_record_offset":9696,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":9696,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":9696,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":10940,"parent_record_offset":10916,"ancestor_record_offset":9696,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":9984,"parent_record_offset":9948,"ancestor_record_offset":9696,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":9696,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":9696,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":9696,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":10732,"parent_record_offset":10708,"ancestor_record_offset":9696,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":9696,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":9696,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":389,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":9696,"inlinee":11460,"source_line":95,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":9696,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11216,"parent_record_offset":11180,"ancestor_record_offset":9696,"inlinee":24402,"source_line":108,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11148,"parent_record_offset":11116,"ancestor_record_offset":9696,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":9696,"inlinee":11459,"source_line":269,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":9696,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":389,"record_offset":11180,"parent_record_offset":11148,"ancestor_record_offset":9696,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":9696,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":9696,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":17708,"parent_record_offset":17676,"ancestor_record_offset":9696,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":9696,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":11744,"parent_record_offset":11712,"ancestor_record_offset":9696,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":389,"record_offset":13624,"parent_record_offset":13592,"ancestor_record_offset":9696,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":9696,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":17576,"parent_record_offset":17552,"ancestor_record_offset":9696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":9696,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":389,"record_offset":17368,"parent_record_offset":17308,"ancestor_record_offset":9696,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":11080,"parent_record_offset":10940,"ancestor_record_offset":9696,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":17424,"parent_record_offset":17400,"ancestor_record_offset":9696,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":389,"record_offset":17400,"parent_record_offset":17308,"ancestor_record_offset":9696,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13680,"parent_record_offset":13592,"ancestor_record_offset":9696,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11396,"parent_record_offset":11368,"ancestor_record_offset":9696,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11712,"parent_record_offset":11688,"ancestor_record_offset":9696,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11860,"parent_record_offset":11688,"ancestor_record_offset":9696,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":9696,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":17516,"parent_record_offset":17488,"ancestor_record_offset":9696,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":13712,"parent_record_offset":13680,"ancestor_record_offset":9696,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F51
// 首条位置: {"module":389,"inlinee":24401}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":10940,"parent_record_offset":10916,"ancestor_record_offset":10916,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":11344,"parent_record_offset":11312,"ancestor_record_offset":11312,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":10916,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":10916,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":10916,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":10968,"parent_record_offset":10940,"ancestor_record_offset":10916,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":389,"record_offset":11368,"parent_record_offset":11344,"ancestor_record_offset":11312,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":11080,"parent_record_offset":10940,"ancestor_record_offset":10916,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$104::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":389,"record_offset":11396,"parent_record_offset":11368,"ancestor_record_offset":11312,"inlinee":8505,"source_line":17,"archive":"_pdb/B/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F54
// 首条位置: {"module":389,"inlinee":24404}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":389,"record_offset":17516,"parent_record_offset":17488,"ancestor_record_offset":17488,"inlinee":19230,"source_line":396,"archive":"_pdb/B/modules/00389.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::systems::paths::draw_path::PathRastersResources", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::add_point_with_mask_to_output
// 信息 ID: c6f32a5211e5f91c7b1bcd56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=10916; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=11312; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=11952; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=12448; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=13072; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=14060; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=14564; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=15720; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::arch_function
// 信息 ID: 5a1b95d00062cb6026b2358b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=17488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections
// 信息 ID: 96ee9c391d95b934ab0a8ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c6001441e9b61c81b53e8aca: void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x4361cf -> 0x74c720 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x4e669b -> 0x74c720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections::closure$0
// 信息 ID: 1af0bbe2bc137ab5561d6712
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=7684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::sample_wall_path_intersections::threshold_curve_path_mask
// 信息 ID: 23751c97ca0c305c45244def
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections; module=389, inline_record=9696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
