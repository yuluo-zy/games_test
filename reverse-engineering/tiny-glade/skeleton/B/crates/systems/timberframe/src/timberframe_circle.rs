// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/timberframe/src/timberframe_circle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2ef20c8395285b4b45fbc0061a43f47431fb216cccc7d6bca2ff1803ae7ac9c3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: timberframe_circle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2294, 3485, 4349
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use std; // namespace candidate
//   package identities: []
// use timberframe; // namespace candidate
//   package identities: [{"name":"timberframe","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3485,"record_offset":260,"rva":35350432,"va":5404059552}
// 编译实例 RVAs: 0x21b67a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21418999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8092cb9fe","size":5,"soft_fail":false,"target_va":5382644528,"va":5404063522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28980767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8e1c945fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5404062266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28547325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e803674cfe","size":5,"soft_fail":false,"target_va":5375516032,"va":5404063352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26318643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8cd686efe","size":5,"soft_fail":false,"target_va":5377742176,"va":5404060814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22126887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8d95eaefe","size":5,"soft_fail":false,"target_va":5381934528,"va":5404061410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22126089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8f761aefe","size":5,"soft_fail":false,"target_va":5381934560,"va":5404060644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b5d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8d4f4ffff","size":5,"soft_fail":false,"target_va":5404056832,"va":5404059687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1421b6000
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e829f3ffff","size":5,"soft_fail":false,"target_va":5404057600,"va":5404060882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14258e8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4028785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e871793d00","size":5,"soft_fail":false,"target_va":5408090336,"va":5404061546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7787396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e884d37600","size":5,"soft_fail":false,"target_va":5411847152,"va":5404059751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7477627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e87b197200","size":5,"soft_fail":false,"target_va":5411541776,"va":5404064144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7418333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8dd317100","size":5,"soft_fail":false,"target_va":5411482433,"va":5404064095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7478271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8ff1b7200","size":5,"soft_fail":false,"target_va":5411542064,"va":5404063788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t7478140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e87c1b7200","size":5,"soft_fail":false,"target_va":5411541904,"va":5404063759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-26031615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e801ca72fe","size":5,"soft_fail":false,"target_va":5378030704,"va":5404062314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-23600509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e883e297fe","size":5,"soft_fail":false,"target_va":5380460416,"va":5404060920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7428254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e89e587100","size":5,"soft_fail":false,"target_va":5411492368,"va":5404064109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7430668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e80c627100","size":5,"soft_fail":false,"target_va":5411494800,"va":5404064127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22236237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8b3b3acfe","size":5,"soft_fail":false,"target_va":5381823600,"va":5404059832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22239527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8d9a6acfe","size":5,"soft_fail":false,"target_va":5381824016,"va":5404063538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_points2_ptcount
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22146905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8a710aefe","size":5,"soft_fail":false,"target_va":5381913504,"va":5404060404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22148643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8dd09aefe","size":5,"soft_fail":false,"target_va":5381911968,"va":5404060606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22147353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8e70eaefe","size":5,"soft_fail":false,"target_va":5381912944,"va":5404060292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22177280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404059552,"raw_hex":"e8009aadfe","size":5,"soft_fail":false,"target_va":5381884480,"va":5404061755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5404059552,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5404059771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3485,"record_offset":6452,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3485,"record_offset":1984,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3485,"record_offset":1284,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":2332,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":8640,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3485,"record_offset":3252,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3485,"record_offset":3392,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3485,"record_offset":2388,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3485,"record_offset":2360,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3485,"record_offset":3672,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":792,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":756,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3485,"record_offset":6220,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3485,"record_offset":3308,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3485,"record_offset":516,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3485,"record_offset":856,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3485,"record_offset":5176,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3485,"record_offset":5088,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":5980,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":7316,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":3728,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":7228,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":5000,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3485,"record_offset":3784,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3485,"record_offset":8672,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3485,"record_offset":8556,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] timberframe::timberframe_circle::construct_timberframe_circle::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"timberframe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3485,"record_offset":3280,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":116771,"source_line":38,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3485,"record_offset":2848,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3485,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3485,"record_offset":6380,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3485,"record_offset":6320,"parent_record_offset":6292,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3485,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3485,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3485,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":260,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3485,"record_offset":1604,"parent_record_offset":1372,"ancestor_record_offset":260,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3485,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3485,"record_offset":1348,"parent_record_offset":1308,"ancestor_record_offset":260,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3485,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":260,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3485,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":260,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3485,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":260,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":3485,"record_offset":4624,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":260,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":2732,"parent_record_offset":2708,"ancestor_record_offset":260,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3485,"record_offset":584,"parent_record_offset":544,"ancestor_record_offset":260,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3485,"record_offset":3160,"parent_record_offset":2872,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3485,"record_offset":2960,"parent_record_offset":2908,"ancestor_record_offset":260,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3485,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":260,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3485,"record_offset":2772,"parent_record_offset":2620,"ancestor_record_offset":260,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":3485,"record_offset":2800,"parent_record_offset":2620,"ancestor_record_offset":260,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3485,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":260,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3485,"record_offset":2068,"parent_record_offset":2044,"ancestor_record_offset":260,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3485,"record_offset":2044,"parent_record_offset":2012,"ancestor_record_offset":260,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3485,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":260,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3485,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":260,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3485,"record_offset":3848,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3485,"record_offset":4016,"parent_record_offset":3948,"ancestor_record_offset":260,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3485,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":260,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":3485,"record_offset":2424,"parent_record_offset":2388,"ancestor_record_offset":260,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::sources::once::once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\sources\once.rs
//     evidence={"module":3485,"record_offset":4708,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":14445,"source_line":56,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3485,"record_offset":4736,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3485,"record_offset":4532,"parent_record_offset":4508,"ancestor_record_offset":260,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3485,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":260,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3485,"record_offset":1420,"parent_record_offset":1372,"ancestor_record_offset":260,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3485,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":260,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3485,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3485,"record_offset":3752,"parent_record_offset":3728,"ancestor_record_offset":260,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3485,"record_offset":5332,"parent_record_offset":5200,"ancestor_record_offset":260,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3485,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":260,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3485,"record_offset":5116,"parent_record_offset":5088,"ancestor_record_offset":260,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3485,"record_offset":2552,"parent_record_offset":2520,"ancestor_record_offset":260,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3485,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":260,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3485,"record_offset":2660,"parent_record_offset":2620,"ancestor_record_offset":260,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":260,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":1116,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":2520,"parent_record_offset":2388,"ancestor_record_offset":260,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":2124,"parent_record_offset":2044,"ancestor_record_offset":260,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":928,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":1152,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3485,"record_offset":4792,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3485,"record_offset":8728,"parent_record_offset":8696,"ancestor_record_offset":260,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":3485,"record_offset":8756,"parent_record_offset":8696,"ancestor_record_offset":260,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3485,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3485,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3485,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3485,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":260,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3485,"record_offset":4588,"parent_record_offset":4532,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3485,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3485,"record_offset":696,"parent_record_offset":668,"ancestor_record_offset":260,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3485,"record_offset":1476,"parent_record_offset":1372,"ancestor_record_offset":260,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3485,"record_offset":1828,"parent_record_offset":1372,"ancestor_record_offset":260,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3485,"record_offset":1088,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3485,"record_offset":1532,"parent_record_offset":1504,"ancestor_record_offset":260,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3485,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":260,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3485,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":260,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3485,"record_offset":2248,"parent_record_offset":2224,"ancestor_record_offset":260,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3485,"record_offset":2200,"parent_record_offset":2148,"ancestor_record_offset":260,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3485,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":260,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3485,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":8968,"parent_record_offset":8904,"ancestor_record_offset":260,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":1000,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":260,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":8992,"parent_record_offset":8968,"ancestor_record_offset":260,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":3485,"record_offset":8604,"parent_record_offset":8580,"ancestor_record_offset":260,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3485,"record_offset":544,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3485,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":260,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3485,"record_offset":8876,"parent_record_offset":8848,"ancestor_record_offset":260,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":3485,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":260,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":3485,"record_offset":5468,"parent_record_offset":5424,"ancestor_record_offset":260,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3485,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":260,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3485,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":260,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3485,"record_offset":2908,"parent_record_offset":2872,"ancestor_record_offset":260,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3485,"record_offset":5264,"parent_record_offset":5232,"ancestor_record_offset":260,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3485,"record_offset":5232,"parent_record_offset":5200,"ancestor_record_offset":260,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3485,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":260,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":6092,"parent_record_offset":6004,"ancestor_record_offset":260,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":7380,"parent_record_offset":7344,"ancestor_record_offset":260,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":260,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":4648,"parent_record_offset":4624,"ancestor_record_offset":260,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3485,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":260,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":3485,"record_offset":4508,"parent_record_offset":3784,"ancestor_record_offset":260,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3485,"record_offset":8784,"parent_record_offset":8696,"ancestor_record_offset":260,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":3485,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":260,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":3485,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":260,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3485,"record_offset":964,"parent_record_offset":856,"ancestor_record_offset":260,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3485,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":260,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3485,"record_offset":1056,"parent_record_offset":964,"ancestor_record_offset":260,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":3485,"record_offset":8940,"parent_record_offset":8904,"ancestor_record_offset":260,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3485,"record_offset":5296,"parent_record_offset":5264,"ancestor_record_offset":260,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] timberframe::timberframe_circle::construct_timberframe_circle::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"timberframe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3485,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":260,"inlinee":116772,"source_line":84,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] timberframe::timberframe_circle::construct_timberframe_circle::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"timberframe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3485,"record_offset":7188,"parent_record_offset":7164,"ancestor_record_offset":260,"inlinee":93340,"source_line":109,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3485,"record_offset":2620,"parent_record_offset":2584,"ancestor_record_offset":260,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3485,"record_offset":2584,"parent_record_offset":2552,"ancestor_record_offset":260,"inlinee":14216,"source_line":196,"archive":"_pdb/B/modules/03485.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C823
// 首条位置: {"module":3485,"inlinee":116771}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C824
// 首条位置: {"module":3485,"inlinee":116772}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16C9C
// 首条位置: {"module":2294,"inlinee":93340}
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
// 完整结构及原始类型记录: timberframe_circle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: timberframe_circle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: timberframe_circle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>", "bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle
// 信息 ID: c57f3fe09f789dc805dc4860
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] a72519a552cbd941be91101e: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)> > @ 0x1a443de -> 0x21b67a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)> > > @ 0x1a46e00 -> 0x21b67a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$0
// 信息 ID: 5ee116e05606d23f9eba708a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=3280; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=6192; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=8528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$1
// 信息 ID: 00e7d6e4b24e916652c59e81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// [闭包环境类型引用] timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=4468; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=4349, inline_record=1508; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=4349, inline_record=2620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$2
// 信息 ID: c8a74399da0faae3078a44fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3485, inline_record=7188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
