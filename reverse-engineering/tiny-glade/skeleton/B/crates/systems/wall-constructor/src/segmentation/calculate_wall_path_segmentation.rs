// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d38c105d07892b28a5e5a88c8c79840e0b96cb82600aaa72cee99a3f239e4154
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: calculate_wall_path_segmentation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 390, 1413, 1415, 1417, 1452, 1455, 3011
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":5068,"rva":7665568,"va":5376374688}
// 编译实例 RVAs: 0x74f7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1293955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e87d41ecff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376375438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1295339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8153cecff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376376838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5507000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8b8075400","size":5,"soft_fail":false,"target_va":5381883040,"va":5376376035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11195041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8a1d2aa00","size":5,"soft_fail":false,"target_va":5387572400,"va":5376377354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11195758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e86ed5aa00","size":5,"soft_fail":false,"target_va":5387573456,"va":5376377693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35470604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e80c3d1d02","size":5,"soft_fail":false,"target_va":5411847184,"va":5376376575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2019230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e86230e1ff","size":5,"soft_fail":false,"target_va":5374357104,"va":5376376329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35163840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8c08e1802","size":5,"soft_fail":false,"target_va":5411540963,"va":5376377118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35164485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e845911802","size":5,"soft_fail":false,"target_va":5411541584,"va":5376377094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35164632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8d8911802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376377139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8e8f6ffff","size":5,"soft_fail":false,"target_va":5376372448,"va":5376374771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35168507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8fba01802","size":5,"soft_fail":false,"target_va":5411543360,"va":5376374848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4041343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e87faa3d00","size":5,"soft_fail":false,"target_va":5380416976,"va":5376375628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11205346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8e2faaa00","size":5,"soft_fail":false,"target_va":5387580992,"va":5376375641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5010261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e855734c00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376375270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5015450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e89a874c00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376375185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5009502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e85e704c00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376376973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5009949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e81d724c00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376374750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35263731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8f3141a02","size":5,"soft_fail":false,"target_va":5411640768,"va":5376377032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11186578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e892b1aa00","size":5,"soft_fail":false,"target_va":5387564432,"va":5376377849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35160017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e8d17f1802","size":5,"soft_fail":false,"target_va":5411537104,"va":5376377082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e863040000","size":5,"soft_fail":false,"target_va":5376377888,"va":5376376760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::new_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11207954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e81205ab00","size":5,"soft_fail":false,"target_va":5387584592,"va":5376376633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::new_loop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11207749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376374688,"raw_hex":"e84504ab00","size":5,"soft_fail":false,"target_va":5387584400,"va":5376376646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9268,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9940,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":10056,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":10000,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":390,"record_offset":10164,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":9324,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":7884,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":7744,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":9876,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":10392,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":390,"record_offset":9848,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":390,"record_offset":9820,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":390,"record_offset":9296,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":390,"record_offset":9356,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":6760,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":5328,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":5784,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":9912,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4631,"source_line":1078,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":10336,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":24417,"source_line":175,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::threshold_range::threshold_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/threshold_range.rs
//     evidence={"module":390,"record_offset":9524,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":24413,"source_line":3,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":5068,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":5068,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":10648,"parent_record_offset":10624,"ancestor_record_offset":5068,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":8356,"parent_record_offset":8148,"ancestor_record_offset":5068,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":10624,"parent_record_offset":10536,"ancestor_record_offset":5068,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":5068,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":8088,"parent_record_offset":8060,"ancestor_record_offset":5068,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":5068,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":5068,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":5068,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":5068,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":8148,"parent_record_offset":8116,"ancestor_record_offset":5068,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":5068,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":5068,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":8184,"parent_record_offset":8148,"ancestor_record_offset":5068,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9668,"parent_record_offset":9632,"ancestor_record_offset":5068,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":5068,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9632,"parent_record_offset":9572,"ancestor_record_offset":5068,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":8032,"parent_record_offset":7992,"ancestor_record_offset":5068,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":8060,"parent_record_offset":8032,"ancestor_record_offset":5068,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":7920,"parent_record_offset":7884,"ancestor_record_offset":5068,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":8916,"parent_record_offset":8884,"ancestor_record_offset":5068,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":390,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":5068,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":390,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":5068,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":390,"record_offset":8952,"parent_record_offset":8916,"ancestor_record_offset":5068,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":390,"record_offset":8460,"parent_record_offset":7992,"ancestor_record_offset":5068,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":390,"record_offset":7956,"parent_record_offset":7920,"ancestor_record_offset":5068,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":390,"record_offset":7992,"parent_record_offset":7956,"ancestor_record_offset":5068,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":390,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":5068,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":390,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":5068,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7096,"parent_record_offset":7068,"ancestor_record_offset":5068,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":5068,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7124,"parent_record_offset":7096,"ancestor_record_offset":5068,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7044,"parent_record_offset":7016,"ancestor_record_offset":5068,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":5068,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7240,"parent_record_offset":7068,"ancestor_record_offset":5068,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":390,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":5068,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":7840,"parent_record_offset":7816,"ancestor_record_offset":5068,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":8756,"parent_record_offset":8704,"ancestor_record_offset":5068,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":5068,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":390,"record_offset":8312,"parent_record_offset":8256,"ancestor_record_offset":5068,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":390,"record_offset":8544,"parent_record_offset":8516,"ancestor_record_offset":5068,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":390,"record_offset":8848,"parent_record_offset":8600,"ancestor_record_offset":5068,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":390,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":5068,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":390,"record_offset":8572,"parent_record_offset":8544,"ancestor_record_offset":5068,"inlinee":4682,"source_line":100,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":390,"record_offset":8600,"parent_record_offset":8572,"ancestor_record_offset":5068,"inlinee":4684,"source_line":277,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":390,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":5068,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":8784,"parent_record_offset":8704,"ancestor_record_offset":5068,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":8704,"parent_record_offset":8660,"ancestor_record_offset":5068,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":8660,"parent_record_offset":8600,"ancestor_record_offset":5068,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":8516,"parent_record_offset":8488,"ancestor_record_offset":5068,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":8884,"parent_record_offset":8848,"ancestor_record_offset":5068,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":390,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":5068,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":8256,"parent_record_offset":8232,"ancestor_record_offset":5068,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":8284,"parent_record_offset":8256,"ancestor_record_offset":5068,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":9604,"parent_record_offset":9572,"ancestor_record_offset":5068,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":5068,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":10592,"parent_record_offset":10564,"ancestor_record_offset":5068,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":390,"record_offset":5484,"parent_record_offset":5452,"ancestor_record_offset":5068,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":5068,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":9088,"parent_record_offset":9052,"ancestor_record_offset":5068,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":9380,"parent_record_offset":9356,"ancestor_record_offset":5068,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":390,"record_offset":6912,"parent_record_offset":6868,"ancestor_record_offset":5068,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":390,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":5068,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":390,"record_offset":7700,"parent_record_offset":6868,"ancestor_record_offset":5068,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":7304,"parent_record_offset":7016,"ancestor_record_offset":5068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":390,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":5068,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":390,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":5068,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":390,"record_offset":10220,"parent_record_offset":10164,"ancestor_record_offset":5068,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":390,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":5068,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":390,"record_offset":6380,"parent_record_offset":6352,"ancestor_record_offset":5068,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":9968,"parent_record_offset":9940,"ancestor_record_offset":5068,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":5068,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":5068,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5068,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":6452,"parent_record_offset":6424,"ancestor_record_offset":5068,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":6100,"parent_record_offset":5984,"ancestor_record_offset":5068,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":390,"record_offset":5452,"parent_record_offset":5424,"ancestor_record_offset":5068,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":390,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":5068,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":390,"record_offset":5952,"parent_record_offset":5920,"ancestor_record_offset":5068,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":390,"record_offset":5920,"parent_record_offset":5884,"ancestor_record_offset":5068,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":390,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":5068,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":390,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":5068,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":390,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":5068,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":390,"record_offset":6424,"parent_record_offset":5884,"ancestor_record_offset":5068,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5424,"parent_record_offset":5392,"ancestor_record_offset":5068,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5392,"parent_record_offset":5364,"ancestor_record_offset":5068,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5584,"parent_record_offset":5552,"ancestor_record_offset":5068,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5680,"parent_record_offset":5584,"ancestor_record_offset":5068,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":5068,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5364,"parent_record_offset":5328,"ancestor_record_offset":5068,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":390,"record_offset":5704,"parent_record_offset":5680,"ancestor_record_offset":5068,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":390,"record_offset":5984,"parent_record_offset":5952,"ancestor_record_offset":5068,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":390,"record_offset":6268,"parent_record_offset":5984,"ancestor_record_offset":5068,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":390,"record_offset":6240,"parent_record_offset":5984,"ancestor_record_offset":5068,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":390,"record_offset":6044,"parent_record_offset":5984,"ancestor_record_offset":5068,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":9052,"parent_record_offset":9020,"ancestor_record_offset":5068,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":9132,"parent_record_offset":9020,"ancestor_record_offset":5068,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":6796,"parent_record_offset":6760,"ancestor_record_offset":5068,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":10972,"parent_record_offset":10940,"ancestor_record_offset":5068,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":11472,"parent_record_offset":11212,"ancestor_record_offset":5068,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":390,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":5068,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":7016,"parent_record_offset":6868,"ancestor_record_offset":5068,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":390,"record_offset":7276,"parent_record_offset":7016,"ancestor_record_offset":5068,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":390,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":5068,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":390,"record_offset":6868,"parent_record_offset":6832,"ancestor_record_offset":5068,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":390,"record_offset":6832,"parent_record_offset":6796,"ancestor_record_offset":5068,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":390,"record_offset":9020,"parent_record_offset":8848,"ancestor_record_offset":5068,"inlinee":24409,"source_line":79,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":390,"record_offset":9572,"parent_record_offset":9524,"ancestor_record_offset":5068,"inlinee":24414,"source_line":109,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::impl$0::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11468
// 首条位置: {"module":1415,"inlinee":70760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":19028,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1415,"record_offset":20168,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1415,"record_offset":16460,"parent_record_offset":16384,"ancestor_record_offset":16384,"inlinee":70762,"source_line":52,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17024,"parent_record_offset":17000,"ancestor_record_offset":16384,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17048,"parent_record_offset":17024,"ancestor_record_offset":16384,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":16384,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":17000,"parent_record_offset":16824,"ancestor_record_offset":16384,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1415,"record_offset":18720,"parent_record_offset":18632,"ancestor_record_offset":16384,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":16384,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18660,"parent_record_offset":18632,"ancestor_record_offset":16384,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":16384,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16824,"parent_record_offset":16792,"ancestor_record_offset":16384,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16792,"parent_record_offset":16764,"ancestor_record_offset":16384,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":16384,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1415,"record_offset":16860,"parent_record_offset":16824,"ancestor_record_offset":16384,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":17240,"parent_record_offset":17208,"ancestor_record_offset":16384,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16644,"parent_record_offset":16608,"ancestor_record_offset":16384,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16608,"parent_record_offset":16560,"ancestor_record_offset":16384,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1415,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":16384,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":17108,"parent_record_offset":16560,"ancestor_record_offset":16384,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":16736,"parent_record_offset":16644,"ancestor_record_offset":16384,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":16524,"parent_record_offset":16460,"ancestor_record_offset":16384,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1415,"record_offset":17988,"parent_record_offset":17956,"ancestor_record_offset":16384,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1415,"record_offset":16884,"parent_record_offset":16860,"ancestor_record_offset":16384,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1415,"record_offset":16908,"parent_record_offset":16884,"ancestor_record_offset":16384,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1415,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":16384,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1415,"record_offset":19384,"parent_record_offset":19360,"ancestor_record_offset":16384,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1415,"record_offset":17688,"parent_record_offset":17652,"ancestor_record_offset":16384,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1415,"record_offset":17652,"parent_record_offset":17240,"ancestor_record_offset":16384,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1415,"record_offset":17908,"parent_record_offset":17240,"ancestor_record_offset":16384,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":16384,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":16956,"parent_record_offset":16932,"ancestor_record_offset":16384,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1415,"record_offset":18688,"parent_record_offset":18660,"ancestor_record_offset":16384,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1415,"record_offset":16680,"parent_record_offset":16644,"ancestor_record_offset":16384,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1415,"record_offset":18560,"parent_record_offset":16460,"ancestor_record_offset":16384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1415,"record_offset":17828,"parent_record_offset":17804,"ancestor_record_offset":16384,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1415,"record_offset":17752,"parent_record_offset":17716,"ancestor_record_offset":16384,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17804,"parent_record_offset":17780,"ancestor_record_offset":16384,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17716,"parent_record_offset":17688,"ancestor_record_offset":16384,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1415,"record_offset":17780,"parent_record_offset":17716,"ancestor_record_offset":16384,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::place_pillars::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":1415,"record_offset":17876,"parent_record_offset":17240,"ancestor_record_offset":16384,"inlinee":70764,"source_line":43,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation
// 记录形式: compiled_procedure; instances=1
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":2052,"rva":7664400,"va":5376373520}
// 编译实例 RVAs: 0x74f310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1697532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e80419e6ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376373863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1292823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e8e945ecff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376374322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fc1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11193418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e84accaa00","size":5,"soft_fail":false,"target_va":5387567584,"va":5376374161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd4b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11198131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e8b3deaa00","size":5,"soft_fail":false,"target_va":5387572400,"va":5376374264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd8d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11198777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e839e1aa00","size":5,"soft_fail":false,"target_va":5387573456,"va":5376374674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35167432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e8c89c1802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376374339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::init_this_update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3378477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e82d8d3300","size":5,"soft_fail":false,"target_va":5379752576,"va":5376374094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3378909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e8dd8e3300","size":5,"soft_fail":false,"target_va":5379753024,"va":5376374110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376373520,"raw_hex":"e874020000","size":5,"soft_fail":false,"target_va":5376374688,"va":5376374055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":390,"record_offset":3308,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":390,"record_offset":2968,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":390,"record_offset":2272,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":5036,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":4500,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":390,"record_offset":3364,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":390,"record_offset":4472,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":4388,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":4756,"parent_record_offset":4668,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":4696,"parent_record_offset":4668,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2052,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":2052,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":2588,"parent_record_offset":2360,"ancestor_record_offset":2052,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":390,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":2052,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":390,"record_offset":2336,"parent_record_offset":2296,"ancestor_record_offset":2052,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":390,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":2052,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":390,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2052,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":390,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":2052,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":2052,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":390,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":2052,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":390,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2052,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":390,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":2052,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":390,"record_offset":4212,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":390,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2052,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":390,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":2052,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":390,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2052,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":2544,"parent_record_offset":2516,"ancestor_record_offset":2052,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":2404,"parent_record_offset":2360,"ancestor_record_offset":2052,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":390,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":2052,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":390,"record_offset":3856,"parent_record_offset":3828,"ancestor_record_offset":2052,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2052,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":3100,"parent_record_offset":3020,"ancestor_record_offset":2052,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":390,"record_offset":2860,"parent_record_offset":2836,"ancestor_record_offset":2052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":390,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":2052,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":390,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2052,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":390,"record_offset":2460,"parent_record_offset":2360,"ancestor_record_offset":2052,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":390,"record_offset":2812,"parent_record_offset":2360,"ancestor_record_offset":2052,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":2516,"parent_record_offset":2488,"ancestor_record_offset":2052,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":2488,"parent_record_offset":2460,"ancestor_record_offset":2052,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":390,"record_offset":2836,"parent_record_offset":2812,"ancestor_record_offset":2052,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":2052,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":3176,"parent_record_offset":3124,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":390,"record_offset":3200,"parent_record_offset":3176,"ancestor_record_offset":2052,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":390,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2052,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":390,"record_offset":3796,"parent_record_offset":3768,"ancestor_record_offset":2052,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":390,"record_offset":3828,"parent_record_offset":3796,"ancestor_record_offset":2052,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":390,"record_offset":3768,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":390,"record_offset":3628,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":390,"record_offset":4240,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":390,"record_offset":3712,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":390,"record_offset":3600,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":390,"record_offset":4120,"parent_record_offset":4096,"ancestor_record_offset":2052,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":390,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2052,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":390,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":2052,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":390,"record_offset":4096,"parent_record_offset":3928,"ancestor_record_offset":2052,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":390,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":2052,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":4036,"parent_record_offset":3952,"ancestor_record_offset":2052,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":2052,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":4332,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":2052,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2052,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3468,"parent_record_offset":3412,"ancestor_record_offset":2052,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":2052,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3572,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":390,"record_offset":3928,"parent_record_offset":3516,"ancestor_record_offset":2052,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F59
// 首条位置: {"module":390,"inlinee":24409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":9052,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":9132,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":9020,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":9088,"parent_record_offset":9052,"ancestor_record_offset":9020,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F5E
// 首条位置: {"module":390,"inlinee":24414}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9632,"parent_record_offset":9572,"ancestor_record_offset":9572,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":9604,"parent_record_offset":9572,"ancestor_record_offset":9572,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":9572,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":9572,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":9572,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9668,"parent_record_offset":9632,"ancestor_record_offset":9572,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":9780,"parent_record_offset":9632,"ancestor_record_offset":9572,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":12816,"rva":7668768,"va":5376377888}
// 编译实例 RVAs: 0x750420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1296712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8b836ecff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376378195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1298722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8de2eecff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376380221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1296496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e89037ecff","size":5,"soft_fail":false,"target_va":5375081536,"va":5376378027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fb2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11184827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8bbaaaa00","size":5,"soft_fail":false,"target_va":5387563696,"va":5376378864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141200c70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11207381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8d502ab00","size":5,"soft_fail":false,"target_va":5387586672,"va":5376379286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11236287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8bf73ab00","size":5,"soft_fail":false,"target_va":5387616352,"va":5376380060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35468909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e86d361d02","size":5,"soft_fail":false,"target_va":5411847200,"va":5376378286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35159992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8b87f1802","size":5,"soft_fail":false,"target_va":5411540963,"va":5376380966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35160718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e88e821802","size":5,"soft_fail":false,"target_va":5411541017,"va":5376380294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35163545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8998d1802","size":5,"soft_fail":false,"target_va":5411541776,"va":5376378226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35161851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8fb861802","size":5,"soft_fail":false,"target_va":5411542422,"va":5376380566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35161464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e878851802","size":5,"soft_fail":false,"target_va":5411542064,"va":5376380595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t35161521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8b1851802","size":5,"soft_fail":false,"target_va":5411542296,"va":5376380770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35162726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8668a1802","size":5,"soft_fail":false,"target_va":5411543584,"va":5376380853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35162704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8508a1802","size":5,"soft_fail":false,"target_va":5411543456,"va":5376380747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35161492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e894851802","size":5,"soft_fail":false,"target_va":5411541904,"va":5376380407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e82c070000","size":5,"soft_fail":false,"target_va":5376381504,"va":5376379663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11206343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376377888,"raw_hex":"e8c7feaa00","size":5,"soft_fail":false,"target_va":5387584336,"va":5376377988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16096,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16128,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":5532,"source_line":1927,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":13080,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16684,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":390,"record_offset":14236,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":16600,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":15064,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":16904,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":17268,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":390,"record_offset":16876,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::PillarProposals::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":390,"record_offset":17240,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24434,"source_line":82,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":390,"record_offset":16380,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24426,"source_line":362,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":390,"record_offset":14552,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24422,"source_line":338,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":13420,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24420,"source_line":1305,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_coord_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":16940,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24428,"source_line":164,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::measure_length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":17036,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24431,"source_line":414,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":13052,"parent_record_offset":12816,"ancestor_record_offset":12816,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":12816,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":12816,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":12816,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":12816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":13876,"parent_record_offset":13700,"ancestor_record_offset":12816,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":13288,"parent_record_offset":13196,"ancestor_record_offset":12816,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":17432,"parent_record_offset":17340,"ancestor_record_offset":12816,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":16228,"parent_record_offset":16196,"ancestor_record_offset":12816,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13640,"parent_record_offset":13612,"ancestor_record_offset":12816,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":12816,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":12816,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":17340,"parent_record_offset":17316,"ancestor_record_offset":12816,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":16276,"parent_record_offset":16252,"ancestor_record_offset":12816,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":16252,"parent_record_offset":16228,"ancestor_record_offset":12816,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12816,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13668,"parent_record_offset":13640,"ancestor_record_offset":12816,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":12816,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":17316,"parent_record_offset":17292,"ancestor_record_offset":12816,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":13232,"parent_record_offset":13196,"ancestor_record_offset":12816,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16196,"parent_record_offset":16128,"ancestor_record_offset":12816,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":12816,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":13976,"parent_record_offset":13576,"ancestor_record_offset":12816,"inlinee":4762,"source_line":3126,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16168,"parent_record_offset":16128,"ancestor_record_offset":12816,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":16348,"parent_record_offset":16128,"ancestor_record_offset":12816,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":15232,"parent_record_offset":15160,"ancestor_record_offset":12816,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":13612,"parent_record_offset":13576,"ancestor_record_offset":12816,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":15096,"parent_record_offset":15064,"ancestor_record_offset":12816,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":15960,"parent_record_offset":15928,"ancestor_record_offset":12816,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":390,"record_offset":16028,"parent_record_offset":15960,"ancestor_record_offset":12816,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":390,"record_offset":15628,"parent_record_offset":15160,"ancestor_record_offset":12816,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":390,"record_offset":13576,"parent_record_offset":13544,"ancestor_record_offset":12816,"inlinee":4761,"source_line":14,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":390,"record_offset":13108,"parent_record_offset":13080,"ancestor_record_offset":12816,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":390,"record_offset":15128,"parent_record_offset":15096,"ancestor_record_offset":12816,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":390,"record_offset":15160,"parent_record_offset":15128,"ancestor_record_offset":12816,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":390,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":12816,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":390,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":12816,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":390,"record_offset":14368,"parent_record_offset":14264,"ancestor_record_offset":12816,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":17092,"parent_record_offset":17060,"ancestor_record_offset":12816,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":15032,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":14072,"parent_record_offset":14044,"ancestor_record_offset":12816,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":14948,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":14976,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":390,"record_offset":13484,"parent_record_offset":13420,"ancestor_record_offset":12816,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":390,"record_offset":16316,"parent_record_offset":16128,"ancestor_record_offset":12816,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":390,"record_offset":17148,"parent_record_offset":17124,"ancestor_record_offset":12816,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":15856,"parent_record_offset":15780,"ancestor_record_offset":12816,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":12816,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":14016,"parent_record_offset":13976,"ancestor_record_offset":12816,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":15204,"parent_record_offset":15160,"ancestor_record_offset":12816,"inlinee":10996,"source_line":853,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$100::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\accum.rs
//     evidence={"module":390,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":12816,"inlinee":17977,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":15700,"parent_record_offset":15676,"ancestor_record_offset":12816,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":15676,"parent_record_offset":15652,"ancestor_record_offset":12816,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":15928,"parent_record_offset":15700,"ancestor_record_offset":12816,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":17124,"parent_record_offset":17036,"ancestor_record_offset":12816,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":17180,"parent_record_offset":17036,"ancestor_record_offset":12816,"inlinee":9237,"source_line":3521,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":12816,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":16500,"parent_record_offset":16452,"ancestor_record_offset":12816,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":13832,"parent_record_offset":13808,"ancestor_record_offset":12816,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":12816,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":17400,"parent_record_offset":17368,"ancestor_record_offset":12816,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":390,"record_offset":14656,"parent_record_offset":14620,"ancestor_record_offset":12816,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":14820,"parent_record_offset":14792,"ancestor_record_offset":12816,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":390,"record_offset":14312,"parent_record_offset":14264,"ancestor_record_offset":12816,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":390,"record_offset":14264,"parent_record_offset":14236,"ancestor_record_offset":12816,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":13456,"parent_record_offset":13420,"ancestor_record_offset":12816,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":390,"record_offset":14480,"parent_record_offset":14392,"ancestor_record_offset":12816,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":17292,"parent_record_offset":17268,"ancestor_record_offset":12816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":12816,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":14108,"parent_record_offset":13976,"ancestor_record_offset":12816,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":390,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":12816,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":12816,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":16712,"parent_record_offset":16684,"ancestor_record_offset":12816,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":14424,"parent_record_offset":14392,"ancestor_record_offset":12816,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":390,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":12816,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":12816,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14856,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14792,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":390,"record_offset":14912,"parent_record_offset":14880,"ancestor_record_offset":12816,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":16452,"parent_record_offset":16380,"ancestor_record_offset":12816,"inlinee":24420,"source_line":1305,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::iter_coords
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":12816,"inlinee":24432,"source_line":345,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$11::sub
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":16976,"parent_record_offset":16940,"ancestor_record_offset":12816,"inlinee":24430,"source_line":1358,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":15004,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":14620,"parent_record_offset":14552,"ancestor_record_offset":12816,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1145D
// 首条位置: {"module":1415,"inlinee":70749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1415,"record_offset":7964,"parent_record_offset":7908,"ancestor_record_offset":7908,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F66
// 首条位置: {"module":390,"inlinee":24422}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":15032,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":14948,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":14976,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14856,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14792,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":15004,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":14620,"parent_record_offset":14552,"ancestor_record_offset":14552,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":390,"record_offset":14656,"parent_record_offset":14620,"ancestor_record_offset":14552,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":14820,"parent_record_offset":14792,"ancestor_record_offset":14552,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":14552,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":390,"record_offset":14912,"parent_record_offset":14880,"ancestor_record_offset":14552,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F6A
// 首条位置: {"module":390,"inlinee":24426}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":390,"record_offset":16452,"parent_record_offset":16380,"ancestor_record_offset":16380,"inlinee":24420,"source_line":1305,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":16424,"parent_record_offset":16380,"ancestor_record_offset":16380,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":390,"record_offset":16564,"parent_record_offset":16452,"ancestor_record_offset":16380,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":16500,"parent_record_offset":16452,"ancestor_record_offset":16380,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":390,"record_offset":16528,"parent_record_offset":16452,"ancestor_record_offset":16380,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
// 记录形式: compiled_procedure; instances=1
// 源码行: 373
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":18176,"rva":7672384,"va":5376381504}
// 编译实例 RVAs: 0x751240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1569009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e80f0fe8ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5376382428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1300616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e87827ecff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376382115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11171814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e8e677aa00","size":5,"soft_fail":false,"target_va":5387553808,"va":5376381989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11175503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e84f86aa00","size":5,"soft_fail":false,"target_va":5387557152,"va":5376381644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fb2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11181668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e8649eaa00","size":5,"soft_fail":false,"target_va":5387563696,"va":5376382023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141206650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11228049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e89153ab00","size":5,"soft_fail":false,"target_va":5387609680,"va":5376381626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35466186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e8ca2b1d02","size":5,"soft_fail":false,"target_va":5411847904,"va":5376381713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35159757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e8cd7e1802","size":5,"soft_fail":false,"target_va":5411542064,"va":5376382302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35159965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e89d7f1802","size":5,"soft_fail":false,"target_va":5411542296,"va":5376382326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35159623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e8477e1802","size":5,"soft_fail":false,"target_va":5411541904,"va":5376382276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11202328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376381504,"raw_hex":"e818efaa00","size":5,"soft_fail":false,"target_va":5387584336,"va":5376382003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":18492,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":390,"record_offset":18380,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":18892,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":390,"record_offset":18464,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":390,"record_offset":18652,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":18864,"parent_record_offset":18176,"ancestor_record_offset":18176,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":18176,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":390,"record_offset":19080,"parent_record_offset":18988,"ancestor_record_offset":18176,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":19016,"parent_record_offset":18988,"ancestor_record_offset":18176,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":18176,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":18964,"parent_record_offset":18940,"ancestor_record_offset":18176,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":18404,"parent_record_offset":18380,"ancestor_record_offset":18176,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":18740,"parent_record_offset":18716,"ancestor_record_offset":18176,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":18764,"parent_record_offset":18740,"ancestor_record_offset":18176,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":390,"record_offset":18556,"parent_record_offset":18520,"ancestor_record_offset":18176,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":18804,"parent_record_offset":18652,"ancestor_record_offset":18176,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":390,"record_offset":18832,"parent_record_offset":18652,"ancestor_record_offset":18176,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":18588,"parent_record_offset":18520,"ancestor_record_offset":18176,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":390,"record_offset":18520,"parent_record_offset":18492,"ancestor_record_offset":18176,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":390,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":18176,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":18176,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":19048,"parent_record_offset":19016,"ancestor_record_offset":18176,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":390,"record_offset":18716,"parent_record_offset":18692,"ancestor_record_offset":18176,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":390,"record_offset":18692,"parent_record_offset":18652,"ancestor_record_offset":18176,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":18176,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate
// 记录形式: compiled_procedure; instances=1
// 源码行: 420
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":19396,"rva":7673360,"va":5376382480}
// 编译实例 RVAs: 0x751610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141207310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11230201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8f95bab00","size":5,"soft_fail":false,"target_va":5387612944,"va":5376382738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412075e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11230904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8b85eab00","size":5,"soft_fail":false,"target_va":5387613664,"va":5376382755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141208d10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: calculate_wall_path_segmentation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11236804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8c475ab00","size":5,"soft_fail":false,"target_va":5387619600,"va":5376382791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35158830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e82e7b1802","size":5,"soft_fail":false,"target_va":5411542064,"va":5376383229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35159020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8ec7b1802","size":5,"soft_fail":false,"target_va":5411542296,"va":5376383271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35160296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8e8801802","size":5,"soft_fail":false,"target_va":5411543584,"va":5376383283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35160156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e85c801802","size":5,"soft_fail":false,"target_va":5411543456,"va":5376383295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11201762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376382480,"raw_hex":"e8e2ecaa00","size":5,"soft_fail":false,"target_va":5387584336,"va":5376382569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":20824,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":20760,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":19876,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":19632,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::start
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":19848,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24438,"source_line":333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":19604,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$8::index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":19904,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24424,"source_line":226,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":19992,"parent_record_offset":19396,"ancestor_record_offset":19396,"inlinee":24440,"source_line":184,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":20724,"parent_record_offset":19992,"ancestor_record_offset":19396,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":390,"record_offset":20376,"parent_record_offset":20032,"ancestor_record_offset":19396,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":390,"record_offset":19944,"parent_record_offset":19904,"ancestor_record_offset":19396,"inlinee":24425,"source_line":3058,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20656,"parent_record_offset":20624,"ancestor_record_offset":19396,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20500,"parent_record_offset":20464,"ancestor_record_offset":19396,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":390,"record_offset":20104,"parent_record_offset":20072,"ancestor_record_offset":19396,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":390,"record_offset":20848,"parent_record_offset":20824,"ancestor_record_offset":19396,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20284,"parent_record_offset":20256,"ancestor_record_offset":19396,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20436,"parent_record_offset":20408,"ancestor_record_offset":19396,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20464,"parent_record_offset":20436,"ancestor_record_offset":19396,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20624,"parent_record_offset":20436,"ancestor_record_offset":19396,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":390,"record_offset":20072,"parent_record_offset":20032,"ancestor_record_offset":19396,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":390,"record_offset":20880,"parent_record_offset":20848,"ancestor_record_offset":19396,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$1::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":390,"record_offset":19676,"parent_record_offset":19632,"ancestor_record_offset":19396,"inlinee":24437,"source_line":51,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20256,"parent_record_offset":20032,"ancestor_record_offset":19396,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20408,"parent_record_offset":19992,"ancestor_record_offset":19396,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopSlice::len
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":390,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":19396,"inlinee":24419,"source_line":199,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":390,"record_offset":20032,"parent_record_offset":19992,"ancestor_record_offset":19396,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1158C
// 首条位置: {"module":1452,"inlinee":71052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19E6B
// 首条位置: {"module":3011,"inlinee":106091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: calculate_wall_path_segmentation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$11::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>", "constructor": "alloc::vec::impl$11::clone", "arguments": ["system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: calculate_wall_path_segmentation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: calculate_wall_path_segmentation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split
// 信息 ID: da9da19265dd543948843ee2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=16380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_split_with_offset
// 信息 ID: e95e28f746bfefc67e3dd911
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment; module=390, inline_record=14552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::add_splits_in_empty_range
// 信息 ID: 77e20112bf0d8bd525abfeab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x750b0f -> 0x751240 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment @ 0x750b70 -> 0x751240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation
// 信息 ID: c64ee1cab159649e1886388c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x74f527 -> 0x74f7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$0
// 信息 ID: b02c18fe02a3acea02987bdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=9020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
// 信息 ID: 9d52c7709410254fabc5cad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=9572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation
// 信息 ID: ca2afd247a1682ae1e1c4afa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 70f0116707fba14b1ced213c: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x173a9d -> 0x74f310 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x42e70b -> 0x74f310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::impl$0::clone
// 信息 ID: 07ba98289ccd92f06d7a7ae3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$11::clone<system_wall_constructor::segmentation::calculate_wall_path_segmentation::WallPathSegmentationMask,alloc::alloc::Global>; module=1415, inline_record=16384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
// 信息 ID: e8a5dd4f3ca1b29a0e7db7c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x74ffb8 -> 0x750420 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2> @ 0x11ff948 -> 0x750420 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8> @ 0x11ffe08 -> 0x750420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure$0
// 信息 ID: 0bcf11f8cef341c9d8bc4582
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0 <- alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; module=1415, inline_record=7908; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0>; module=1415, inline_record=8084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate
// 信息 ID: c239827e732f12774102bf9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0> @ 0x1200d3c -> 0x751610 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::Vec::retain<usize,alloc::alloc::Global,system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment::closure_env$0> @ 0x1200dfa -> 0x751610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$0
// 信息 ID: b4dc862bd57f2734ffdc0afc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- utils::curve::curve_slice::CurveSlice::extend_left_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,tuple$<tuple$<usize,glam::f32::vec3::Vec3>,tuple$<usize,glam::f32::vec3::Vec3> > >; module=1452, inline_record=4752; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=7204; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=7964; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<take_until::take_until::TakeUntil<core::iter::adapters::rev::Rev<core::ops::range::RangeInclusive<usize> >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> >,core::iter::adapters::skip::Skip<core::iter::adapters::rev::Rev<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > > >,utils::curve::curve_slice::impl$6::extend_left_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$0> > > > >; module=1455, inline_record=8868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure$1
// 信息 ID: 9306d5c8d3629f9d1fe43263
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,glam::f32::vec3::Vec3>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1 <- take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] take_until::take_until::impl$2::next<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1> >; module=3011, inline_record=1524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
