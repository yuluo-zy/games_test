// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: ae3e80e2e2a6b36b4a70670a4de355882404da38b02d134e40b89e6cf278695c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_to_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 551, 561, 1639, 1813, 1844, 3393
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":82180,"rva":11635616,"va":5380344736}
// 编译实例 RVAs: 0xb18ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4666415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8d1cbb8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380345114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6866180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8fc3a97ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380345215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1409755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8db821500","size":5,"soft_fail":false,"target_va":5381754944,"va":5380345184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2979849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e809782d00","size":5,"soft_fail":false,"target_va":5383325024,"va":5380345170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415228f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10525168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8f099a000","size":5,"soft_fail":false,"target_va":5390870768,"va":5380345595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160b240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11478437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8a525af00","size":5,"soft_fail":false,"target_va":5391823424,"va":5380344982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36276063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e85f872902","size":5,"soft_fail":false,"target_va":5416621600,"va":5380345532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36276569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e859892902","size":5,"soft_fail":false,"target_va":5416622208,"va":5380345634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1530344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e8e8591700","size":5,"soft_fail":false,"target_va":5381876000,"va":5380345651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-352369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380344736,"raw_hex":"e88f9ffaff","size":5,"soft_fail":false,"target_va":5379992560,"va":5380344924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":86096,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":82368,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":82396,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":561,"record_offset":82432,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":561,"record_offset":85088,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
//     evidence={"module":561,"record_offset":82488,"parent_record_offset":82180,"ancestor_record_offset":82180,"inlinee":35272,"source_line":53,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83524,"parent_record_offset":83500,"ancestor_record_offset":82180,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83548,"parent_record_offset":83524,"ancestor_record_offset":82180,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83572,"parent_record_offset":83548,"ancestor_record_offset":82180,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83500,"parent_record_offset":83412,"ancestor_record_offset":82180,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83440,"parent_record_offset":83412,"ancestor_record_offset":82180,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83412,"parent_record_offset":83388,"ancestor_record_offset":82180,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83388,"parent_record_offset":83364,"ancestor_record_offset":82180,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":82860,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":82892,"parent_record_offset":82860,"ancestor_record_offset":82180,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82832,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82580,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82772,"parent_record_offset":82748,"ancestor_record_offset":82180,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":86120,"parent_record_offset":86096,"ancestor_record_offset":82180,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":85824,"parent_record_offset":85800,"ancestor_record_offset":82180,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":85392,"parent_record_offset":85368,"ancestor_record_offset":82180,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":85476,"parent_record_offset":85452,"ancestor_record_offset":82180,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":83296,"parent_record_offset":83272,"ancestor_record_offset":82180,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":82964,"parent_record_offset":82924,"ancestor_record_offset":82180,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":82796,"parent_record_offset":82772,"ancestor_record_offset":82180,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":82748,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":83468,"parent_record_offset":83440,"ancestor_record_offset":82180,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":85624,"parent_record_offset":85596,"ancestor_record_offset":82180,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":85596,"parent_record_offset":85568,"ancestor_record_offset":82180,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":82632,"parent_record_offset":82608,"ancestor_record_offset":82180,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":82456,"parent_record_offset":82432,"ancestor_record_offset":82180,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":85416,"parent_record_offset":85392,"ancestor_record_offset":82180,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":83340,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":82712,"parent_record_offset":82688,"ancestor_record_offset":82180,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":85740,"parent_record_offset":85716,"ancestor_record_offset":82180,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":82664,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":82688,"parent_record_offset":82664,"ancestor_record_offset":82180,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":561,"record_offset":83020,"parent_record_offset":82924,"ancestor_record_offset":82180,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":561,"record_offset":82924,"parent_record_offset":82892,"ancestor_record_offset":82180,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":561,"record_offset":85848,"parent_record_offset":85824,"ancestor_record_offset":82180,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":83160,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":83240,"parent_record_offset":83184,"ancestor_record_offset":82180,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":82608,"parent_record_offset":82488,"ancestor_record_offset":82180,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":85536,"parent_record_offset":85508,"ancestor_record_offset":82180,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":85568,"parent_record_offset":85536,"ancestor_record_offset":82180,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":85508,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":85368,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":85920,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":85452,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":561,"record_offset":85312,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":561,"record_offset":85800,"parent_record_offset":85776,"ancestor_record_offset":82180,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":85148,"parent_record_offset":85124,"ancestor_record_offset":82180,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":85776,"parent_record_offset":85668,"ancestor_record_offset":82180,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":85180,"parent_record_offset":85148,"ancestor_record_offset":82180,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85716,"parent_record_offset":85692,"ancestor_record_offset":82180,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":86012,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85692,"parent_record_offset":85668,"ancestor_record_offset":82180,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85232,"parent_record_offset":85208,"ancestor_record_offset":82180,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85208,"parent_record_offset":85148,"ancestor_record_offset":82180,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85256,"parent_record_offset":85232,"ancestor_record_offset":82180,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85340,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":85668,"parent_record_offset":85256,"ancestor_record_offset":82180,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":561,"record_offset":85124,"parent_record_offset":85088,"ancestor_record_offset":82180,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":83272,"parent_record_offset":83240,"ancestor_record_offset":82180,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":86064,"parent_record_offset":85088,"ancestor_record_offset":82180,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89C8
// 首条位置: {"module":561,"inlinee":35272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":82860,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82832,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82580,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":82748,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":83340,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":82664,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":83160,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":82608,"parent_record_offset":82488,"ancestor_record_offset":82488,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83524,"parent_record_offset":83500,"ancestor_record_offset":82488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83548,"parent_record_offset":83524,"ancestor_record_offset":82488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83572,"parent_record_offset":83548,"ancestor_record_offset":82488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":83500,"parent_record_offset":83412,"ancestor_record_offset":82488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83440,"parent_record_offset":83412,"ancestor_record_offset":82488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83412,"parent_record_offset":83388,"ancestor_record_offset":82488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":83388,"parent_record_offset":83364,"ancestor_record_offset":82488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":82892,"parent_record_offset":82860,"ancestor_record_offset":82488,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":82772,"parent_record_offset":82748,"ancestor_record_offset":82488,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":83296,"parent_record_offset":83272,"ancestor_record_offset":82488,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":82964,"parent_record_offset":82924,"ancestor_record_offset":82488,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":82796,"parent_record_offset":82772,"ancestor_record_offset":82488,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":83468,"parent_record_offset":83440,"ancestor_record_offset":82488,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":82632,"parent_record_offset":82608,"ancestor_record_offset":82488,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":83364,"parent_record_offset":83340,"ancestor_record_offset":82488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":82712,"parent_record_offset":82688,"ancestor_record_offset":82488,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":82688,"parent_record_offset":82664,"ancestor_record_offset":82488,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":561,"record_offset":83020,"parent_record_offset":82924,"ancestor_record_offset":82488,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":561,"record_offset":82924,"parent_record_offset":82892,"ancestor_record_offset":82488,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":83240,"parent_record_offset":83184,"ancestor_record_offset":82488,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":83184,"parent_record_offset":83160,"ancestor_record_offset":82488,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":83272,"parent_record_offset":83240,"ancestor_record_offset":82488,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8644
// 首条位置: {"module":551,"inlinee":34372}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":72244,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":551,"record_offset":74576,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":74672,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/in_progress_curve.rs
//     evidence={"module":551,"record_offset":72040,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":34374,"source_line":141,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":551,"record_offset":72012,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":551,"record_offset":72844,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":74360,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":74304,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":74604,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":73848,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":75344,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":75660,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":551,"record_offset":72096,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":551,"record_offset":75284,"parent_record_offset":71856,"ancestor_record_offset":71856,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":72672,"parent_record_offset":72648,"ancestor_record_offset":71856,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":72696,"parent_record_offset":72672,"ancestor_record_offset":71856,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":74884,"parent_record_offset":74860,"ancestor_record_offset":71856,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":74908,"parent_record_offset":74884,"ancestor_record_offset":71856,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":72720,"parent_record_offset":72696,"ancestor_record_offset":71856,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":74932,"parent_record_offset":74908,"ancestor_record_offset":71856,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":72648,"parent_record_offset":72464,"ancestor_record_offset":71856,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":74860,"parent_record_offset":74768,"ancestor_record_offset":71856,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":72400,"parent_record_offset":72372,"ancestor_record_offset":71856,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":74796,"parent_record_offset":74768,"ancestor_record_offset":71856,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":74768,"parent_record_offset":74744,"ancestor_record_offset":71856,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":72464,"parent_record_offset":72428,"ancestor_record_offset":71856,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":72428,"parent_record_offset":72400,"ancestor_record_offset":71856,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":74744,"parent_record_offset":74720,"ancestor_record_offset":71856,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":72504,"parent_record_offset":72464,"ancestor_record_offset":71856,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":72336,"parent_record_offset":72272,"ancestor_record_offset":71856,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":72308,"parent_record_offset":72272,"ancestor_record_offset":71856,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":72372,"parent_record_offset":72336,"ancestor_record_offset":71856,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":73908,"parent_record_offset":73848,"ancestor_record_offset":71856,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":72272,"parent_record_offset":72244,"ancestor_record_offset":71856,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":72528,"parent_record_offset":72504,"ancestor_record_offset":71856,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":72176,"parent_record_offset":72152,"ancestor_record_offset":71856,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":72200,"parent_record_offset":72176,"ancestor_record_offset":71856,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":73576,"parent_record_offset":73552,"ancestor_record_offset":71856,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":73144,"parent_record_offset":73120,"ancestor_record_offset":71856,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":73228,"parent_record_offset":73204,"ancestor_record_offset":71856,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":551,"record_offset":75308,"parent_record_offset":75284,"ancestor_record_offset":71856,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":551,"record_offset":74500,"parent_record_offset":74476,"ancestor_record_offset":71856,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":73644,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":72608,"parent_record_offset":72552,"ancestor_record_offset":71856,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":72552,"parent_record_offset":72528,"ancestor_record_offset":71856,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":72580,"parent_record_offset":72552,"ancestor_record_offset":71856,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":74828,"parent_record_offset":74796,"ancestor_record_offset":71856,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":73376,"parent_record_offset":73348,"ancestor_record_offset":71856,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":73348,"parent_record_offset":73320,"ancestor_record_offset":71856,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":74232,"parent_record_offset":74208,"ancestor_record_offset":71856,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":74328,"parent_record_offset":74304,"ancestor_record_offset":71856,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":551,"record_offset":72152,"parent_record_offset":72128,"ancestor_record_offset":71856,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":551,"record_offset":72128,"parent_record_offset":72096,"ancestor_record_offset":71856,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":72776,"parent_record_offset":72336,"ancestor_record_offset":71856,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":72800,"parent_record_offset":72776,"ancestor_record_offset":71856,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":74696,"parent_record_offset":74672,"ancestor_record_offset":71856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":73492,"parent_record_offset":73468,"ancestor_record_offset":71856,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":551,"record_offset":74632,"parent_record_offset":74604,"ancestor_record_offset":71856,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":73940,"parent_record_offset":73908,"ancestor_record_offset":71856,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":73972,"parent_record_offset":73940,"ancestor_record_offset":71856,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":551,"record_offset":73600,"parent_record_offset":73576,"ancestor_record_offset":71856,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":74444,"parent_record_offset":74412,"ancestor_record_offset":71856,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":74412,"parent_record_offset":74384,"ancestor_record_offset":71856,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":74384,"parent_record_offset":74360,"ancestor_record_offset":71856,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":75936,"parent_record_offset":75912,"ancestor_record_offset":71856,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":75856,"parent_record_offset":75660,"ancestor_record_offset":71856,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":551,"record_offset":74180,"parent_record_offset":74152,"ancestor_record_offset":71856,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":551,"record_offset":74208,"parent_record_offset":74180,"ancestor_record_offset":71856,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":73288,"parent_record_offset":73260,"ancestor_record_offset":71856,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":73320,"parent_record_offset":73288,"ancestor_record_offset":71856,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":73260,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":73120,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":73672,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":73204,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":73064,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":73552,"parent_record_offset":73528,"ancestor_record_offset":71856,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":72904,"parent_record_offset":72880,"ancestor_record_offset":71856,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":73528,"parent_record_offset":73420,"ancestor_record_offset":71856,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":72932,"parent_record_offset":72904,"ancestor_record_offset":71856,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73468,"parent_record_offset":73444,"ancestor_record_offset":71856,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73764,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73444,"parent_record_offset":73420,"ancestor_record_offset":71856,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":72984,"parent_record_offset":72960,"ancestor_record_offset":71856,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":72960,"parent_record_offset":72904,"ancestor_record_offset":71856,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73008,"parent_record_offset":72984,"ancestor_record_offset":71856,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73092,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":73420,"parent_record_offset":73008,"ancestor_record_offset":71856,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":72880,"parent_record_offset":72844,"ancestor_record_offset":71856,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":551,"record_offset":74476,"parent_record_offset":74444,"ancestor_record_offset":71856,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":75700,"parent_record_offset":75660,"ancestor_record_offset":71856,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":75372,"parent_record_offset":75344,"ancestor_record_offset":71856,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":73816,"parent_record_offset":72844,"ancestor_record_offset":71856,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":72064,"parent_record_offset":72040,"ancestor_record_offset":71856,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":73880,"parent_record_offset":73848,"ancestor_record_offset":71856,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":74016,"parent_record_offset":73848,"ancestor_record_offset":71856,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":551,"record_offset":75912,"parent_record_offset":75660,"ancestor_record_offset":71856,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":551,"record_offset":74152,"parent_record_offset":73848,"ancestor_record_offset":71856,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=33
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B63
// 首条位置: {"module":528,"inlinee":31587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":59024,"parent_record_offset":58632,"ancestor_record_offset":58632,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":58664,"parent_record_offset":58632,"ancestor_record_offset":58632,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":528,"record_offset":58800,"parent_record_offset":58776,"ancestor_record_offset":58632,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":58716,"parent_record_offset":58688,"ancestor_record_offset":58632,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":58744,"parent_record_offset":58688,"ancestor_record_offset":58632,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":58688,"parent_record_offset":58664,"ancestor_record_offset":58632,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":528,"record_offset":58776,"parent_record_offset":58744,"ancestor_record_offset":58632,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00528.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_to_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::drift::sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::stable::drift::sort", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::quicksort::quicksort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::stable::quicksort::quicksort", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_to_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_to_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 信息 ID: f81a85ac4d9906a1617d0e42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments; module=561, inline_record=82488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 信息 ID: 1264d9602423207e74e7d8f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=551, inline_record=71856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 信息 ID: a6bcdfd070b0ac24574f2457
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::driftsort_main<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::drift::sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 8 条。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=528, inline_record=58632; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=528, inline_record=59084; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=528, inline_record=59352; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=528, inline_record=59436; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1639, inline_record=3936; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1639, inline_record=4508; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1639, inline_record=34260; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1639, inline_record=34732; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments
// 信息 ID: 0e72ad60a2c721ad39b84a2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 57ca5d49ac23a91769ba3026: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0x28e88c -> 0xb18ba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> @ 0x53785c -> 0xb18ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
