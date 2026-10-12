// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ae3e80e2e2a6b36b4a70670a4de355882404da38b02d134e40b89e6cf278695c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_to_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 551, 574, 578, 1592, 1654, 1736, 3208
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
// 首条位置: {"module":551,"record_offset":52896,"rva":10925536,"va":5379634656}
// 编译实例 RVAs: 0xa6b5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4958814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8a255b4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5379635145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4553548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8b484baff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379635047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-532847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e891def7ff","size":5,"soft_fail":false,"target_va":5379102272,"va":5379635114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b4c550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t920520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8c80b0e00","size":5,"soft_fail":false,"target_va":5380556112,"va":5379635587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2299424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e820162300","size":5,"soft_fail":false,"target_va":5381934528,"va":5379635099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fba80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8979351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e897038900","size":5,"soft_fail":false,"target_va":5388614272,"va":5379634916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413323f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9202346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8aa6a8c00","size":5,"soft_fail":false,"target_va":5388837872,"va":5379635521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31906301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8fdd9e601","size":5,"soft_fail":false,"target_va":5411541776,"va":5379635470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31906500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e8c4dae601","size":5,"soft_fail":false,"target_va":5411542064,"va":5379635559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t238439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379634656,"raw_hex":"e867a30300","size":5,"soft_fail":false,"target_va":5379873296,"va":5379634852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":56544,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":53084,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":53112,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":551,"record_offset":53148,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":26671,"source_line":35,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":551,"record_offset":55448,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
//     evidence={"module":551,"record_offset":53212,"parent_record_offset":52896,"ancestor_record_offset":52896,"inlinee":31559,"source_line":53,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":54220,"parent_record_offset":54196,"ancestor_record_offset":52896,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":54196,"parent_record_offset":54108,"ancestor_record_offset":52896,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54136,"parent_record_offset":54108,"ancestor_record_offset":52896,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54108,"parent_record_offset":54084,"ancestor_record_offset":52896,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54084,"parent_record_offset":54060,"ancestor_record_offset":52896,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":53584,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":53616,"parent_record_offset":53584,"ancestor_record_offset":52896,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53556,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53304,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53496,"parent_record_offset":53472,"ancestor_record_offset":52896,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":551,"record_offset":56572,"parent_record_offset":56544,"ancestor_record_offset":52896,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":56268,"parent_record_offset":56244,"ancestor_record_offset":52896,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":55776,"parent_record_offset":55752,"ancestor_record_offset":52896,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":55860,"parent_record_offset":55836,"ancestor_record_offset":52896,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":55800,"parent_record_offset":55776,"ancestor_record_offset":52896,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":53688,"parent_record_offset":53648,"ancestor_record_offset":52896,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":551,"record_offset":53520,"parent_record_offset":53496,"ancestor_record_offset":52896,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":53472,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":54164,"parent_record_offset":54136,"ancestor_record_offset":52896,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":56008,"parent_record_offset":55980,"ancestor_record_offset":52896,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":55980,"parent_record_offset":55952,"ancestor_record_offset":52896,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":53356,"parent_record_offset":53332,"ancestor_record_offset":52896,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":53176,"parent_record_offset":53148,"ancestor_record_offset":52896,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":54036,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":53436,"parent_record_offset":53412,"ancestor_record_offset":52896,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":56128,"parent_record_offset":56104,"ancestor_record_offset":52896,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":53388,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":551,"record_offset":53412,"parent_record_offset":53388,"ancestor_record_offset":52896,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":551,"record_offset":53744,"parent_record_offset":53648,"ancestor_record_offset":52896,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":551,"record_offset":53648,"parent_record_offset":53616,"ancestor_record_offset":52896,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":551,"record_offset":56292,"parent_record_offset":56268,"ancestor_record_offset":52896,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53884,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53964,"parent_record_offset":53908,"ancestor_record_offset":52896,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53332,"parent_record_offset":53212,"ancestor_record_offset":52896,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":55920,"parent_record_offset":55892,"ancestor_record_offset":52896,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":55952,"parent_record_offset":55920,"ancestor_record_offset":52896,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":55892,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":55752,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":56364,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":55836,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":551,"record_offset":55724,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":551,"record_offset":56244,"parent_record_offset":56220,"ancestor_record_offset":52896,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":55508,"parent_record_offset":55484,"ancestor_record_offset":52896,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":55532,"parent_record_offset":55508,"ancestor_record_offset":52896,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":56220,"parent_record_offset":56052,"ancestor_record_offset":52896,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":551,"record_offset":55564,"parent_record_offset":55532,"ancestor_record_offset":52896,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":56160,"parent_record_offset":56076,"ancestor_record_offset":52896,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":56104,"parent_record_offset":56076,"ancestor_record_offset":52896,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":56456,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":56076,"parent_record_offset":56052,"ancestor_record_offset":52896,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":55616,"parent_record_offset":55592,"ancestor_record_offset":52896,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":55592,"parent_record_offset":55532,"ancestor_record_offset":52896,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":55640,"parent_record_offset":55616,"ancestor_record_offset":52896,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":55696,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":551,"record_offset":56052,"parent_record_offset":55640,"ancestor_record_offset":52896,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":55484,"parent_record_offset":55448,"ancestor_record_offset":52896,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":551,"record_offset":53996,"parent_record_offset":53964,"ancestor_record_offset":52896,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":56512,"parent_record_offset":55448,"ancestor_record_offset":52896,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B47
// 首条位置: {"module":551,"inlinee":31559}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":53584,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53556,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53304,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":53472,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":54036,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":53388,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53884,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53332,"parent_record_offset":53212,"ancestor_record_offset":53212,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":54220,"parent_record_offset":54196,"ancestor_record_offset":53212,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":54196,"parent_record_offset":54108,"ancestor_record_offset":53212,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54136,"parent_record_offset":54108,"ancestor_record_offset":53212,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54108,"parent_record_offset":54084,"ancestor_record_offset":53212,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":54084,"parent_record_offset":54060,"ancestor_record_offset":53212,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":53616,"parent_record_offset":53584,"ancestor_record_offset":53212,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":53496,"parent_record_offset":53472,"ancestor_record_offset":53212,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":53688,"parent_record_offset":53648,"ancestor_record_offset":53212,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":551,"record_offset":53520,"parent_record_offset":53496,"ancestor_record_offset":53212,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":54164,"parent_record_offset":54136,"ancestor_record_offset":53212,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":53356,"parent_record_offset":53332,"ancestor_record_offset":53212,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":54060,"parent_record_offset":54036,"ancestor_record_offset":53212,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":53436,"parent_record_offset":53412,"ancestor_record_offset":53212,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":551,"record_offset":53412,"parent_record_offset":53388,"ancestor_record_offset":53212,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":551,"record_offset":53744,"parent_record_offset":53648,"ancestor_record_offset":53212,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":551,"record_offset":53648,"parent_record_offset":53616,"ancestor_record_offset":53212,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53964,"parent_record_offset":53908,"ancestor_record_offset":53212,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":53908,"parent_record_offset":53884,"ancestor_record_offset":53212,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":551,"record_offset":53996,"parent_record_offset":53964,"ancestor_record_offset":53212,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8116
// 首条位置: {"module":578,"inlinee":33046}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":147876,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":578,"record_offset":150280,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":150376,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/in_progress_curve.rs
//     evidence={"module":578,"record_offset":147644,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":32484,"source_line":141,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":578,"record_offset":147616,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":578,"record_offset":148496,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":150092,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":150036,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":150308,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":149592,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":150936,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":151252,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":578,"record_offset":147700,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":578,"record_offset":150876,"parent_record_offset":147460,"ancestor_record_offset":147460,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":148348,"parent_record_offset":148324,"ancestor_record_offset":147460,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":148372,"parent_record_offset":148348,"ancestor_record_offset":147460,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":150588,"parent_record_offset":150564,"ancestor_record_offset":147460,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":148324,"parent_record_offset":148148,"ancestor_record_offset":147460,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":578,"record_offset":150564,"parent_record_offset":150472,"ancestor_record_offset":147460,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":148088,"parent_record_offset":148060,"ancestor_record_offset":147460,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":150500,"parent_record_offset":150472,"ancestor_record_offset":147460,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":150472,"parent_record_offset":150448,"ancestor_record_offset":147460,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":148148,"parent_record_offset":148116,"ancestor_record_offset":147460,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":148116,"parent_record_offset":148088,"ancestor_record_offset":147460,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":150448,"parent_record_offset":150424,"ancestor_record_offset":147460,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":578,"record_offset":148184,"parent_record_offset":148148,"ancestor_record_offset":147460,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":148020,"parent_record_offset":147984,"ancestor_record_offset":147460,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":578,"record_offset":147984,"parent_record_offset":147912,"ancestor_record_offset":147460,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":147956,"parent_record_offset":147912,"ancestor_record_offset":147460,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":148060,"parent_record_offset":148020,"ancestor_record_offset":147460,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":149652,"parent_record_offset":149592,"ancestor_record_offset":147460,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":578,"record_offset":147912,"parent_record_offset":147876,"ancestor_record_offset":147460,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":578,"record_offset":148208,"parent_record_offset":148184,"ancestor_record_offset":147460,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":578,"record_offset":148232,"parent_record_offset":148208,"ancestor_record_offset":147460,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":147780,"parent_record_offset":147756,"ancestor_record_offset":147460,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":147804,"parent_record_offset":147780,"ancestor_record_offset":147460,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":578,"record_offset":149316,"parent_record_offset":149292,"ancestor_record_offset":147460,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":148824,"parent_record_offset":148800,"ancestor_record_offset":147460,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":578,"record_offset":148908,"parent_record_offset":148884,"ancestor_record_offset":147460,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":578,"record_offset":150900,"parent_record_offset":150876,"ancestor_record_offset":147460,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":578,"record_offset":147844,"parent_record_offset":147700,"ancestor_record_offset":147460,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":578,"record_offset":148448,"parent_record_offset":148424,"ancestor_record_offset":147460,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":578,"record_offset":149384,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":148256,"parent_record_offset":148232,"ancestor_record_offset":147460,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":148280,"parent_record_offset":148256,"ancestor_record_offset":147460,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":578,"record_offset":150532,"parent_record_offset":150500,"ancestor_record_offset":147460,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":149056,"parent_record_offset":149028,"ancestor_record_offset":147460,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":578,"record_offset":149028,"parent_record_offset":149000,"ancestor_record_offset":147460,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":578,"record_offset":149964,"parent_record_offset":149940,"ancestor_record_offset":147460,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":578,"record_offset":150060,"parent_record_offset":150036,"ancestor_record_offset":147460,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":578,"record_offset":147756,"parent_record_offset":147732,"ancestor_record_offset":147460,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":578,"record_offset":147732,"parent_record_offset":147700,"ancestor_record_offset":147460,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":578,"record_offset":148424,"parent_record_offset":148020,"ancestor_record_offset":147460,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":578,"record_offset":150400,"parent_record_offset":150376,"ancestor_record_offset":147460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":578,"record_offset":149176,"parent_record_offset":149152,"ancestor_record_offset":147460,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":578,"record_offset":150336,"parent_record_offset":150308,"ancestor_record_offset":147460,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":149684,"parent_record_offset":149652,"ancestor_record_offset":147460,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":578,"record_offset":149716,"parent_record_offset":149684,"ancestor_record_offset":147460,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":578,"record_offset":149340,"parent_record_offset":149316,"ancestor_record_offset":147460,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":150176,"parent_record_offset":150144,"ancestor_record_offset":147460,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":150144,"parent_record_offset":150116,"ancestor_record_offset":147460,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":150116,"parent_record_offset":150092,"ancestor_record_offset":147460,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":151528,"parent_record_offset":151504,"ancestor_record_offset":147460,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":578,"record_offset":151448,"parent_record_offset":151252,"ancestor_record_offset":147460,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":578,"record_offset":149912,"parent_record_offset":149884,"ancestor_record_offset":147460,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":578,"record_offset":149940,"parent_record_offset":149912,"ancestor_record_offset":147460,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":148968,"parent_record_offset":148940,"ancestor_record_offset":147460,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":149000,"parent_record_offset":148968,"ancestor_record_offset":147460,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":578,"record_offset":148940,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":148800,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":149412,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":578,"record_offset":148884,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":578,"record_offset":148772,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":578,"record_offset":149292,"parent_record_offset":149268,"ancestor_record_offset":147460,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":578,"record_offset":148556,"parent_record_offset":148532,"ancestor_record_offset":147460,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":578,"record_offset":148580,"parent_record_offset":148556,"ancestor_record_offset":147460,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":578,"record_offset":149268,"parent_record_offset":149100,"ancestor_record_offset":147460,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":578,"record_offset":148612,"parent_record_offset":148580,"ancestor_record_offset":147460,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":149208,"parent_record_offset":149124,"ancestor_record_offset":147460,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":149152,"parent_record_offset":149124,"ancestor_record_offset":147460,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":149504,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":149124,"parent_record_offset":149100,"ancestor_record_offset":147460,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":148664,"parent_record_offset":148640,"ancestor_record_offset":147460,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":148640,"parent_record_offset":148580,"ancestor_record_offset":147460,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":148688,"parent_record_offset":148664,"ancestor_record_offset":147460,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":148744,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":578,"record_offset":149100,"parent_record_offset":148688,"ancestor_record_offset":147460,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":578,"record_offset":148532,"parent_record_offset":148496,"ancestor_record_offset":147460,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":578,"record_offset":150208,"parent_record_offset":150176,"ancestor_record_offset":147460,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":151292,"parent_record_offset":151252,"ancestor_record_offset":147460,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":150964,"parent_record_offset":150936,"ancestor_record_offset":147460,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":149560,"parent_record_offset":148496,"ancestor_record_offset":147460,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":147668,"parent_record_offset":147644,"ancestor_record_offset":147460,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":149624,"parent_record_offset":149592,"ancestor_record_offset":147460,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":578,"record_offset":149760,"parent_record_offset":149592,"ancestor_record_offset":147460,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":578,"record_offset":151504,"parent_record_offset":151252,"ancestor_record_offset":147460,"inlinee":25387,"source_line":68,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":578,"record_offset":149884,"parent_record_offset":149592,"ancestor_record_offset":147460,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8029
// 首条位置: {"module":574,"inlinee":32809}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":574,"record_offset":1060,"parent_record_offset":1020,"ancestor_record_offset":1020,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":1100,"parent_record_offset":1020,"ancestor_record_offset":1020,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":574,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":1020,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":1180,"parent_record_offset":1124,"ancestor_record_offset":1020,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=44
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":574,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":1020,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00574.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":574,"record_offset":1212,"parent_record_offset":1180,"ancestor_record_offset":1020,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00574.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::stable::merge::merge", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::stable::driftsort_main<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global> >", "constructor": "core::slice::sort::stable::driftsort_main", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::insert_tail", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::sort4_stable", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::smallsort::small_sort_general_with_scratch", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::median3_rec", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::shared::pivot::choose_pivot<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::shared::pivot::choose_pivot", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 17; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >,bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> > >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera> >", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > > >", "bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleGrabParams>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_edit_circle::CircleSignifierParams>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::main_camera::MainCameraMoved>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::wall::wall_highlight::HighlightWallCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::wall::shapes::circle::signifiers::edit_arrow::EditCircleSignifier>,ref_mut$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::systems::wall::shapes::circle::signifiers::edit_line::SignifierEditCircleShapeLine>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref$<country_core::systems::ui::signifiers::SignifierAnimation> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_edit_circle::AnimatedCircleSignifierResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>,bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "tuple$<country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>", "bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>", "bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnCircleRotated>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::roofs::commands::RecalculateRoofCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::RotateCircleCache>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::wall::shapes::circle::ui_rotate_circle::CurrentAngleGizmo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 信息 ID: 038b3e7705223cd217fd65b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments; module=551, inline_record=53212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 信息 ID: 86a6686d4abdb24baf5f7131
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=578, inline_record=147460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 信息 ID: 6309dd1e96a9cab984a061a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::driftsort_main<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::vec::Vec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::shared::pivot::choose_pivot<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::stable::drift::sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=574, inline_record=1020; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::merge::merge<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=574, inline_record=1612; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=3600; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insert_tail<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=4116; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=18144; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=18560; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=19076; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1592, inline_record=19516; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments
// 信息 ID: 1241a643ba6f28bd2c32c675
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 57ca5d49ac23a91769ba3026: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0x32446b -> 0xa6b5e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > > @ 0x44168d -> 0xa6b5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
