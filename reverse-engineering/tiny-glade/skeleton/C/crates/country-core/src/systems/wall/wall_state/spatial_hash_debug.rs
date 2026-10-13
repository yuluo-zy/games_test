// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/wall_state/spatial_hash_debug.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 7ecb3d38e2a9b56467620e90438ef5ece085c5599282c2e31bc6227213d47466
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: spatial_hash_debug.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 550, 567
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":550,"record_offset":27192,"rva":11284624,"va":5379993744}
// 编译实例 RVAs: 0xac3090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4315336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e83827beff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379994019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-4316044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e87424beff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379994743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11271817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8770154ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379994004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140906710
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1821494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8ca34e4ff","size":5,"soft_fail":false,"target_va":5378172688,"va":5379994177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7dfd0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-285062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e87aa6fbff","size":5,"soft_fail":false,"target_va":5379710928,"va":5379995985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3d5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t499995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e81ba10700","size":5,"soft_fail":false,"target_va":5380494784,"va":5379994784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3db60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t501616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e870a70700","size":5,"soft_fail":false,"target_va":5380496224,"va":5379994603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140debf80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3313246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e85e8e3200","size":5,"soft_fail":false,"target_va":5383307136,"va":5379993885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3329639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e867ce3200","size":5,"soft_fail":false,"target_va":5383325056,"va":5379995412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152cb10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10916532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8b492a600","size":5,"soft_fail":false,"target_va":5390912272,"va":5379995735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: spatial_hash_debug.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37062798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e88e883502","size":5,"soft_fail":false,"target_va":5417057792,"va":5379994989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36625090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8c2da2e02","size":5,"soft_fail":false,"target_va":5416620769,"va":5379995674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36626611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8b3e02e02","size":5,"soft_fail":false,"target_va":5416622560,"va":5379995944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_rem_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36626660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8e4e02e02","size":5,"soft_fail":false,"target_va":5416622624,"va":5379995959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugRectangle2d::show
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-318582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e88a23fbff","size":5,"soft_fail":false,"target_va":5379675360,"va":5379993937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3312150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8168a3200","size":5,"soft_fail":false,"target_va":5383306800,"va":5379994645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3311351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e8f7863200","size":5,"soft_fail":false,"target_va":5383305920,"va":5379994564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3325310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379993744,"raw_hex":"e87ebd3200","size":5,"soft_fail":false,"target_va":5383320512,"va":5379995197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":550,"record_offset":27992,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":29052,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":30112,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":550,"record_offset":30276,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":550,"record_offset":27512,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":550,"record_offset":27596,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":29164,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":27432,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":34231,"source_line":545,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_index_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":30340,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":34229,"source_line":527,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":30472,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":33017,"source_line":538,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::dims
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":27568,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":34230,"source_line":552,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":29456,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":34240,"source_line":162,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":550,"record_offset":28164,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":33494,"source_line":425,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":550,"record_offset":29108,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":33404,"source_line":403,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::show
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":550,"record_offset":28192,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":28669,"source_line":446,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugRectangle2d::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":550,"record_offset":30860,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":33481,"source_line":355,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugRectangle2d::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":550,"record_offset":27540,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":33477,"source_line":346,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":30744,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":30168,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":550,"record_offset":27816,"parent_record_offset":27192,"ancestor_record_offset":27192,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":28064,"parent_record_offset":28040,"ancestor_record_offset":27192,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":27192,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":29352,"parent_record_offset":29328,"ancestor_record_offset":27192,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":27192,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":27192,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":27192,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":28040,"parent_record_offset":28016,"ancestor_record_offset":27192,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":550,"record_offset":29328,"parent_record_offset":29236,"ancestor_record_offset":27192,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":550,"record_offset":28016,"parent_record_offset":27992,"ancestor_record_offset":27192,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":550,"record_offset":33172,"parent_record_offset":33148,"ancestor_record_offset":27192,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29772,"parent_record_offset":29748,"ancestor_record_offset":27192,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29264,"parent_record_offset":29236,"ancestor_record_offset":27192,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29236,"parent_record_offset":29212,"ancestor_record_offset":27192,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29820,"parent_record_offset":29796,"ancestor_record_offset":27192,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":27192,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":29212,"parent_record_offset":29188,"ancestor_record_offset":27192,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":29748,"parent_record_offset":29612,"ancestor_record_offset":27192,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":29612,"parent_record_offset":29456,"ancestor_record_offset":27192,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":29720,"parent_record_offset":29612,"ancestor_record_offset":27192,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":29960,"parent_record_offset":29612,"ancestor_record_offset":27192,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":550,"record_offset":30912,"parent_record_offset":30888,"ancestor_record_offset":27192,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":550,"record_offset":30960,"parent_record_offset":30936,"ancestor_record_offset":27192,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":550,"record_offset":27664,"parent_record_offset":27628,"ancestor_record_offset":27192,"inlinee":7964,"source_line":1916,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":550,"record_offset":30408,"parent_record_offset":30384,"ancestor_record_offset":27192,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":550,"record_offset":29576,"parent_record_offset":29536,"ancestor_record_offset":27192,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":550,"record_offset":27696,"parent_record_offset":27628,"ancestor_record_offset":27192,"inlinee":7966,"source_line":195,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":550,"record_offset":27628,"parent_record_offset":27596,"ancestor_record_offset":27192,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":550,"record_offset":29296,"parent_record_offset":29264,"ancestor_record_offset":27192,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":550,"record_offset":27720,"parent_record_offset":27696,"ancestor_record_offset":27192,"inlinee":7967,"source_line":613,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":550,"record_offset":27768,"parent_record_offset":27744,"ancestor_record_offset":27192,"inlinee":7969,"source_line":2498,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":550,"record_offset":27744,"parent_record_offset":27720,"ancestor_record_offset":27192,"inlinee":7968,"source_line":2581,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":550,"record_offset":27932,"parent_record_offset":27816,"ancestor_record_offset":27192,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":550,"record_offset":27844,"parent_record_offset":27816,"ancestor_record_offset":27192,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":550,"record_offset":27480,"parent_record_offset":27456,"ancestor_record_offset":27192,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":550,"record_offset":27876,"parent_record_offset":27816,"ancestor_record_offset":27192,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":29924,"parent_record_offset":29612,"ancestor_record_offset":27192,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":29188,"parent_record_offset":29164,"ancestor_record_offset":27192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":550,"record_offset":30136,"parent_record_offset":30112,"ancestor_record_offset":27192,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":31384,"parent_record_offset":31316,"ancestor_record_offset":27192,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":30304,"parent_record_offset":30276,"ancestor_record_offset":27192,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":31408,"parent_record_offset":31384,"ancestor_record_offset":27192,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::sort_unstable_by_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":550,"record_offset":29500,"parent_record_offset":29456,"ancestor_record_offset":27192,"inlinee":11893,"source_line":3240,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":550,"record_offset":29536,"parent_record_offset":29500,"ancestor_record_offset":27192,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::impl$4::query_with_circle::closure$2
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":550,"record_offset":29860,"parent_record_offset":29612,"ancestor_record_offset":27192,"inlinee":34236,"source_line":174,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":550,"record_offset":27456,"parent_record_offset":27432,"ancestor_record_offset":27192,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":550,"record_offset":30384,"parent_record_offset":30340,"ancestor_record_offset":27192,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8BA2
// 首条位置: {"module":567,"inlinee":35746}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":14740,"parent_record_offset":14712,"ancestor_record_offset":14712,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":14832,"parent_record_offset":14808,"ancestor_record_offset":14712,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":14880,"parent_record_offset":14856,"ancestor_record_offset":14712,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":567,"record_offset":14856,"parent_record_offset":14832,"ancestor_record_offset":14712,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":14808,"parent_record_offset":14768,"ancestor_record_offset":14712,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":14768,"parent_record_offset":14740,"ancestor_record_offset":14712,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":14920,"parent_record_offset":14768,"ancestor_record_offset":14712,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00567.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug
// 信息 ID: b6e392e3fac7518f9edd97cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8294f52627e94b61d29dd75: void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> > @ 0x28afbe -> 0xac3090 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)> @ 0x414d39 -> 0xac3090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure$2
// 信息 ID: 5381eac9d4af6ac569735fe2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2 <- country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env$2>; module=567, inline_record=14712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
