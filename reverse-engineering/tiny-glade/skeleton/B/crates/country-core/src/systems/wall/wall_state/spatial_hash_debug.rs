// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/wall_state/spatial_hash_debug.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7ecb3d38e2a9b56467620e90438ef5ece085c5599282c2e31bc6227213d47466
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spatial_hash_debug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 499, 535
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":499,"record_offset":59912,"rva":9355488,"va":5378064608}
// 编译实例 RVAs: 0x8ec0e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2983386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8267ad2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378064869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2984089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e86777d2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378065588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409f0e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1067600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8504a1000","size":5,"soft_fail":false,"target_va":5379133056,"va":5378065451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cb90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3869214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e81e0a3b00","size":5,"soft_fail":false,"target_va":5381933968,"va":5378064749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3868325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8a5063b00","size":5,"soft_fail":false,"target_va":5381934560,"va":5378066230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33782080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e840790302","size":5,"soft_fail":false,"target_va":5411847904,"va":5378065819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33474498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8c2c7fe01","size":5,"soft_fail":false,"target_va":5411540992,"va":5378066489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33477061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8c5d1fe01","size":5,"soft_fail":false,"target_va":5411543584,"va":5378066518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33476919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e837d1fe01","size":5,"soft_fail":false,"target_va":5411543456,"va":5378066532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1807457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e861941b00","size":5,"soft_fail":false,"target_va":5379873088,"va":5378065626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2198762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8ea8c2100","size":5,"soft_fail":false,"target_va":5380263792,"va":5378065025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle2d::show
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2196650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8aa842100","size":5,"soft_fail":false,"target_va":5380261440,"va":5378064785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3848631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e8b7b93a00","size":5,"soft_fail":false,"target_va":5381914128,"va":5378065492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3846454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e836b13a00","size":5,"soft_fail":false,"target_va":5381911872,"va":5378065413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3851023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378064608,"raw_hex":"e80fc33a00","size":5,"soft_fail":false,"target_va":5381917056,"va":5378066028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":60708,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":499,"record_offset":61652,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":499,"record_offset":62976,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":499,"record_offset":61972,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":499,"record_offset":60256,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":499,"record_offset":60312,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":61736,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":499,"record_offset":60144,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28123,"source_line":545,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_index_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":499,"record_offset":62144,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28130,"source_line":527,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":499,"record_offset":62244,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28131,"source_line":538,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::dims
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":499,"record_offset":60284,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28126,"source_line":552,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":499,"record_offset":60852,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28128,"source_line":391,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":499,"record_offset":61708,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28129,"source_line":369,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugRectangle2d::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":499,"record_offset":62632,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28132,"source_line":321,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugRectangle2d::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":499,"record_offset":60224,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":28125,"source_line":312,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":499,"record_offset":62516,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":499,"record_offset":62036,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":499,"record_offset":60532,"parent_record_offset":59912,"ancestor_record_offset":59912,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":60756,"parent_record_offset":60732,"ancestor_record_offset":59912,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":60780,"parent_record_offset":60756,"ancestor_record_offset":59912,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":61924,"parent_record_offset":61900,"ancestor_record_offset":59912,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":60732,"parent_record_offset":60708,"ancestor_record_offset":59912,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":499,"record_offset":61900,"parent_record_offset":61808,"ancestor_record_offset":59912,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":499,"record_offset":64576,"parent_record_offset":64552,"ancestor_record_offset":59912,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":499,"record_offset":61836,"parent_record_offset":61808,"ancestor_record_offset":59912,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":499,"record_offset":61808,"parent_record_offset":61784,"ancestor_record_offset":59912,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":499,"record_offset":61784,"parent_record_offset":61760,"ancestor_record_offset":59912,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":499,"record_offset":62684,"parent_record_offset":62660,"ancestor_record_offset":59912,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":499,"record_offset":62732,"parent_record_offset":62708,"ancestor_record_offset":59912,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":499,"record_offset":60380,"parent_record_offset":60344,"ancestor_record_offset":59912,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":499,"record_offset":60412,"parent_record_offset":60344,"ancestor_record_offset":59912,"inlinee":15400,"source_line":187,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":499,"record_offset":60344,"parent_record_offset":60312,"ancestor_record_offset":59912,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":499,"record_offset":61868,"parent_record_offset":61836,"ancestor_record_offset":59912,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":499,"record_offset":60436,"parent_record_offset":60412,"ancestor_record_offset":59912,"inlinee":15401,"source_line":545,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":499,"record_offset":60484,"parent_record_offset":60460,"ancestor_record_offset":59912,"inlinee":15403,"source_line":2297,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\int_macros.rs
//     evidence={"module":499,"record_offset":60460,"parent_record_offset":60436,"ancestor_record_offset":59912,"inlinee":15402,"source_line":2379,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":60648,"parent_record_offset":60532,"ancestor_record_offset":59912,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":60560,"parent_record_offset":60532,"ancestor_record_offset":59912,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":60192,"parent_record_offset":60168,"ancestor_record_offset":59912,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":499,"record_offset":60592,"parent_record_offset":60532,"ancestor_record_offset":59912,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":61760,"parent_record_offset":61736,"ancestor_record_offset":59912,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":499,"record_offset":63000,"parent_record_offset":62976,"ancestor_record_offset":59912,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":63100,"parent_record_offset":63032,"ancestor_record_offset":59912,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":499,"record_offset":62000,"parent_record_offset":61972,"ancestor_record_offset":59912,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":63124,"parent_record_offset":63100,"ancestor_record_offset":59912,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":499,"record_offset":60808,"parent_record_offset":60780,"ancestor_record_offset":59912,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":499,"record_offset":60168,"parent_record_offset":60144,"ancestor_record_offset":59912,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":499,"record_offset":62184,"parent_record_offset":62144,"ancestor_record_offset":59912,"inlinee":11009,"source_line":48,"archive":"_pdb/B/modules/00499.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77FC
// 首条位置: {"module":535,"inlinee":30716}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":535,"record_offset":57164,"parent_record_offset":57136,"ancestor_record_offset":57136,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":57224,"parent_record_offset":57200,"ancestor_record_offset":57136,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":57272,"parent_record_offset":57248,"ancestor_record_offset":57136,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":57248,"parent_record_offset":57224,"ancestor_record_offset":57136,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":535,"record_offset":57200,"parent_record_offset":57164,"ancestor_record_offset":57136,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":57312,"parent_record_offset":57164,"ancestor_record_offset":57136,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00535.jsonl.gz"}
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
// 完整结构及原始类型记录: spatial_hash_debug.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: spatial_hash_debug.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: spatial_hash_debug.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug
// 信息 ID: 8a5634f01d6cbefdc6b425a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8294f52627e94b61d29dd75: void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x2516dc -> 0x8ec0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > > @ 0x44e0be -> 0x8ec0e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure$2
// 信息 ID: f14070152d5bf5d83943821e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2 <- country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>; module=535, inline_record=57136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
