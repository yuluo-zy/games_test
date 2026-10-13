// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/threshold_range.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 44d08e679eabfba5736badba798a7aae36dbd52626f7a715b422892cc2a34e67
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: threshold_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 390, 1413
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5F5D
// 首条位置: {"module":390,"inlinee":24413}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":390,"record_offset":9572,"parent_record_offset":9524,"ancestor_record_offset":9524,"inlinee":24414,"source_line":109,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9692,"parent_record_offset":9668,"ancestor_record_offset":9524,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":9524,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":390,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":9524,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9668,"parent_record_offset":9632,"ancestor_record_offset":9524,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":390,"record_offset":9632,"parent_record_offset":9572,"ancestor_record_offset":9524,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":390,"record_offset":9604,"parent_record_offset":9572,"ancestor_record_offset":9524,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":390,"record_offset":9780,"parent_record_offset":9632,"ancestor_record_offset":9524,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1413,"record_offset":2216,"rva":18874160,"va":5387583280}
// 编译实例 RVAs: 0x11fff30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18316161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387583280,"raw_hex":"e87f84e8fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387583820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141238d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t232414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387583280,"raw_hex":"e8de8b0300","size":5,"soft_fail":false,"target_va":5387816320,"va":5387583901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13228970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387583280,"raw_hex":"e8562436ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5387583957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23958978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387583280,"raw_hex":"e8c2956d01","size":5,"soft_fail":false,"target_va":5411543104,"va":5387584121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23959014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387583280,"raw_hex":"e8e6956d01","size":5,"soft_fail":false,"target_va":5411543120,"va":5387584101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1413,"record_offset":2668,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":70721,"source_line":35,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":2216,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2216,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":3368,"parent_record_offset":3344,"ancestor_record_offset":2216,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":3312,"parent_record_offset":3276,"ancestor_record_offset":2216,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":3276,"parent_record_offset":2668,"ancestor_record_offset":2216,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2216,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1413,"record_offset":3160,"parent_record_offset":3132,"ancestor_record_offset":2216,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1413,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":2216,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1413,"record_offset":2944,"parent_record_offset":2900,"ancestor_record_offset":2216,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1413,"record_offset":2788,"parent_record_offset":2752,"ancestor_record_offset":2216,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2216,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1413,"record_offset":2824,"parent_record_offset":2668,"ancestor_record_offset":2216,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1413,"record_offset":3060,"parent_record_offset":3016,"ancestor_record_offset":2216,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1413,"record_offset":3432,"parent_record_offset":3276,"ancestor_record_offset":2216,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1413,"record_offset":2724,"parent_record_offset":2668,"ancestor_record_offset":2216,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1413,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2216,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1413,"record_offset":3016,"parent_record_offset":2980,"ancestor_record_offset":2216,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3196,"parent_record_offset":2900,"ancestor_record_offset":2216,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1413,"record_offset":3132,"parent_record_offset":2900,"ancestor_record_offset":2216,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1413,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2216,"inlinee":70723,"source_line":40,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1413,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":2216,"inlinee":30460,"source_line":1082,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1413,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":2216,"inlinee":30462,"source_line":92,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1413,"record_offset":272,"rva":18872208,"va":5387581328}
// 编译实例 RVAs: 0x11ff790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd4b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e8e5d9ffff","size":5,"soft_fail":false,"target_va":5387572400,"va":5387582150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e84edcffff","size":5,"soft_fail":false,"target_va":5387573008,"va":5387582141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24265049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e859417201","size":5,"soft_fail":false,"target_va":5411847168,"va":5387582114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23757520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e8d0826a01","size":5,"soft_fail":false,"target_va":5411339744,"va":5387582219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7164503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e8a9ad92ff","size":5,"soft_fail":false,"target_va":5380417392,"va":5387581890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11203885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e8d30a55ff","size":5,"soft_fail":false,"target_va":5376377888,"va":5387581768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t197966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e84e050300","size":5,"soft_fail":false,"target_va":5387779984,"va":5387582013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387581328,"raw_hex":"e83d0c0000","size":5,"soft_fail":false,"target_va":5387584784,"va":5387581646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1413,"record_offset":616,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":70717,"source_line":511,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1080,"parent_record_offset":1052,"ancestor_record_offset":272,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":272,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1148,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1112,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":1008,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":1268,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":1052,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":872,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":836,"parent_record_offset":616,"ancestor_record_offset":272,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::forget_remaining_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1413,"record_offset":1300,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":10155,"source_line":157,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1413,"record_offset":964,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1413,"record_offset":904,"parent_record_offset":872,"ancestor_record_offset":272,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1413,"record_offset":932,"parent_record_offset":872,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1413,"record_offset":800,"parent_record_offset":744,"ancestor_record_offset":272,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":1413,"record_offset":744,"parent_record_offset":616,"ancestor_record_offset":272,"inlinee":24554,"source_line":23,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1413,"record_offset":688,"parent_record_offset":616,"ancestor_record_offset":272,"inlinee":24417,"source_line":175,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1413,"record_offset":1344,"rva":18873296,"va":5387582416}
// 编译实例 RVAs: 0x11ffbd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387582416,"raw_hex":"e8abd5ffff","size":5,"soft_fail":false,"target_va":5387572400,"va":5387583232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24264142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387582416,"raw_hex":"e8ce3d7201","size":5,"soft_fail":false,"target_va":5411847184,"va":5387583037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13225141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387582416,"raw_hex":"e84b3336ff","size":5,"soft_fail":false,"target_va":5374357872,"va":5387583008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11205101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387582416,"raw_hex":"e8130655ff","size":5,"soft_fail":false,"target_va":5376377888,"va":5387582984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387582416,"raw_hex":"e878070000","size":5,"soft_fail":false,"target_va":5387584784,"va":5387582867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1413,"record_offset":2000,"parent_record_offset":1344,"ancestor_record_offset":1344,"inlinee":70719,"source_line":113,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$8
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1413,"record_offset":1764,"parent_record_offset":1344,"ancestor_record_offset":1344,"inlinee":70718,"source_line":131,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1876,"parent_record_offset":1844,"ancestor_record_offset":1344,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":1344,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1413,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":1344,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":1844,"parent_record_offset":1804,"ancestor_record_offset":1344,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1413,"record_offset":1804,"parent_record_offset":1764,"ancestor_record_offset":1344,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1413,"record_offset":2148,"parent_record_offset":2104,"ancestor_record_offset":1344,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1413,"record_offset":1964,"parent_record_offset":1804,"ancestor_record_offset":1344,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01413.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1413,"record_offset":2048,"parent_record_offset":2000,"ancestor_record_offset":1344,"inlinee":24417,"source_line":175,"archive":"_pdb/B/modules/01413.jsonl.gz"}
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
// 完整结构及原始类型记录: threshold_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: threshold_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: threshold_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range
// 信息 ID: fcf6372188e35cfad8df8411
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=390, inline_record=9524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 信息 ID: ab9f638a80c6cea76f6ab11c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>", "system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0x1238bcf -> 0x11fff30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1,system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2>
// 信息 ID: 5e169fe3a8526ae20f1a3e22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$1", "system_wall_constructor::construct_platform_stairs::process_loop_range_into_wallpath_segments::closure_env$2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x775140 -> 0x11ff790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 信息 ID: f0cc49114ba0815d718a11a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7", "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs @ 0x752571 -> 0x11ffbd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
