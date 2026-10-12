// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 89a3116aedaae7a9cfc2922d3487b55ac53a08a0366fe243006bbe9b444aeafe
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_tiles_gravity.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 541, 567, 1585, 1632
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":541,"record_offset":124228,"rva":10656528,"va":5379365648}
// 编译实例 RVAs: 0xa29b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9750624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8a0376bff","size":5,"soft_fail":false,"target_va":5369616112,"va":5379366731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0be0
//     owner=unresolved_namespace; count=2
//     target_aliases=251 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4559601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e80f6dbaff","size":5,"soft_fail":false,"target_va":5374807008,"va":5379366604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4285925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e81b9abeff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379367424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896fb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1651110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e85acee6ff","size":5,"soft_fail":false,"target_va":5377716144,"va":5379367249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-788001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8dff9f3ff","size":5,"soft_fail":false,"target_va":5378578208,"va":5379366204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-779679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8611af4ff","size":5,"soft_fail":false,"target_va":5378587184,"va":5379366858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130d8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9320991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e81f3a8e00","size":5,"soft_fail":false,"target_va":5388687584,"va":5379366588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416032a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12424085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e89593bd00","size":5,"soft_fail":false,"target_va":5391790752,"va":5379366662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160aaa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12454541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e88d0abe00","size":5,"soft_fail":false,"target_va":5391821472,"va":5379366926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160c800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12462292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8d428be00","size":5,"soft_fail":false,"target_va":5391828992,"va":5379366695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32480879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e86f9eef01","size":5,"soft_fail":false,"target_va":5411847152,"va":5379366268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: roof_tiles_gravity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32480280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8189cef01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379366883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12779858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e85201c300","size":5,"soft_fail":false,"target_va":5392146176,"va":5379366313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::update_archetypes_unsafe_world_cell::h44f339aee1bbcd98 (.llvm.3979812295465967050)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12772567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8d7e4c200","size":5,"soft_fail":false,"target_va":5392138896,"va":5379366324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::validate_world::panic_mismatched
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32133467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e85b51ea01","size":5,"soft_fail":false,"target_va":5411500544,"va":5379367072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32174759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8a7f2ea01","size":5,"soft_fail":false,"target_va":5411541776,"va":5379367012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32115368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e8a80aea01","size":5,"soft_fail":false,"target_va":5411482433,"va":5379367060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32172546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e802eaea01","size":5,"soft_fail":false,"target_va":5411539504,"va":5379366953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32173197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379365648,"raw_hex":"e88decea01","size":5,"soft_fail":false,"target_va":5411540288,"va":5379367086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379365648,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5379366288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::state::QueryState::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":541,"record_offset":128372,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":30255,"source_line":1185,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::query
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":541,"record_offset":128316,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":30253,"source_line":1535,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":124448,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":541,"record_offset":129164,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::flat_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":541,"record_offset":129108,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":18576,"source_line":1430,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":128280,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":130116,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":541,"record_offset":128040,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::uniform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":541,"record_offset":130012,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":541,"record_offset":129248,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":130304,"parent_record_offset":130280,"ancestor_record_offset":124228,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":541,"record_offset":130280,"parent_record_offset":130188,"ancestor_record_offset":124228,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":127688,"parent_record_offset":127664,"ancestor_record_offset":124228,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":130216,"parent_record_offset":130188,"ancestor_record_offset":124228,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":130188,"parent_record_offset":130164,"ancestor_record_offset":124228,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":127736,"parent_record_offset":127712,"ancestor_record_offset":124228,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":127712,"parent_record_offset":127688,"ancestor_record_offset":124228,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":541,"record_offset":130164,"parent_record_offset":130140,"ancestor_record_offset":124228,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":127664,"parent_record_offset":127640,"ancestor_record_offset":124228,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":127640,"parent_record_offset":127600,"ancestor_record_offset":124228,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":129188,"parent_record_offset":129164,"ancestor_record_offset":124228,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":127600,"parent_record_offset":127576,"ancestor_record_offset":124228,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":541,"record_offset":124504,"parent_record_offset":124448,"ancestor_record_offset":124228,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":541,"record_offset":125496,"parent_record_offset":125456,"ancestor_record_offset":124228,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":541,"record_offset":126880,"parent_record_offset":126856,"ancestor_record_offset":124228,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":541,"record_offset":128768,"parent_record_offset":128744,"ancestor_record_offset":124228,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":541,"record_offset":128800,"parent_record_offset":128768,"ancestor_record_offset":124228,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":541,"record_offset":128400,"parent_record_offset":128372,"ancestor_record_offset":124228,"inlinee":30256,"source_line":326,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":541,"record_offset":128460,"parent_record_offset":128400,"ancestor_record_offset":124228,"inlinee":30258,"source_line":342,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::query_unchecked_manual
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":541,"record_offset":128596,"parent_record_offset":128460,"ancestor_record_offset":124228,"inlinee":30259,"source_line":392,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::update_archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/state.rs
//     evidence={"module":541,"record_offset":128432,"parent_record_offset":128400,"ancestor_record_offset":124228,"inlinee":30257,"source_line":512,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":541,"record_offset":127988,"parent_record_offset":127964,"ancestor_record_offset":124228,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":541,"record_offset":127964,"parent_record_offset":127940,"ancestor_record_offset":124228,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":541,"record_offset":127912,"parent_record_offset":127880,"ancestor_record_offset":124228,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":541,"record_offset":127856,"parent_record_offset":127244,"ancestor_record_offset":124228,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":541,"record_offset":127880,"parent_record_offset":127856,"ancestor_record_offset":124228,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":541,"record_offset":127284,"parent_record_offset":127244,"ancestor_record_offset":124228,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":541,"record_offset":127940,"parent_record_offset":127880,"ancestor_record_offset":124228,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":541,"record_offset":127356,"parent_record_offset":127320,"ancestor_record_offset":124228,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":541,"record_offset":127320,"parent_record_offset":127284,"ancestor_record_offset":124228,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":541,"record_offset":127576,"parent_record_offset":127552,"ancestor_record_offset":124228,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":541,"record_offset":128744,"parent_record_offset":128372,"ancestor_record_offset":124228,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":541,"record_offset":128496,"parent_record_offset":128460,"ancestor_record_offset":124228,"inlinee":4113,"source_line":187,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::query_filtered
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":541,"record_offset":128340,"parent_record_offset":128316,"ancestor_record_offset":124228,"inlinee":30254,"source_line":1559,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":541,"record_offset":128644,"parent_record_offset":128620,"ancestor_record_offset":124228,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::identifier::impl$9::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/identifier.rs
//     evidence={"module":541,"record_offset":128560,"parent_record_offset":128536,"ancestor_record_offset":124228,"inlinee":4117,"source_line":11,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::archetypes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":129068,"parent_record_offset":128768,"ancestor_record_offset":124228,"inlinee":13723,"source_line":263,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":128620,"parent_record_offset":128596,"ancestor_record_offset":124228,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":129304,"parent_record_offset":129276,"ancestor_record_offset":124228,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":129276,"parent_record_offset":129248,"ancestor_record_offset":124228,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":541,"record_offset":127244,"parent_record_offset":124448,"ancestor_record_offset":124228,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":541,"record_offset":124548,"parent_record_offset":124504,"ancestor_record_offset":124228,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":541,"record_offset":125336,"parent_record_offset":125312,"ancestor_record_offset":124228,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":541,"record_offset":128536,"parent_record_offset":128496,"ancestor_record_offset":124228,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":541,"record_offset":125384,"parent_record_offset":125360,"ancestor_record_offset":124228,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":541,"record_offset":128108,"parent_record_offset":128068,"ancestor_record_offset":124228,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":541,"record_offset":124788,"parent_record_offset":124764,"ancestor_record_offset":124228,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":541,"record_offset":124872,"parent_record_offset":124848,"ancestor_record_offset":124228,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":541,"record_offset":124812,"parent_record_offset":124788,"ancestor_record_offset":124228,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::FlatMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":541,"record_offset":129132,"parent_record_offset":129108,"ancestor_record_offset":124228,"inlinee":18578,"source_line":22,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":541,"record_offset":129212,"parent_record_offset":129188,"ancestor_record_offset":124228,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":541,"record_offset":130248,"parent_record_offset":130216,"ancestor_record_offset":124228,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":541,"record_offset":125020,"parent_record_offset":124992,"ancestor_record_offset":124228,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":541,"record_offset":124992,"parent_record_offset":124964,"ancestor_record_offset":124228,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":541,"record_offset":127488,"parent_record_offset":127396,"ancestor_record_offset":124228,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":130052,"parent_record_offset":130012,"ancestor_record_offset":124228,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":127512,"parent_record_offset":127488,"ancestor_record_offset":124228,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":126856,"parent_record_offset":125496,"ancestor_record_offset":124228,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":125456,"parent_record_offset":124504,"ancestor_record_offset":124228,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":541,"record_offset":129980,"parent_record_offset":129248,"ancestor_record_offset":124228,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":126912,"parent_record_offset":125496,"ancestor_record_offset":124228,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":541,"record_offset":129028,"parent_record_offset":129004,"ancestor_record_offset":124228,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":541,"record_offset":125252,"parent_record_offset":125228,"ancestor_record_offset":124228,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":541,"record_offset":128192,"parent_record_offset":128164,"ancestor_record_offset":124228,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":541,"record_offset":128220,"parent_record_offset":128192,"ancestor_record_offset":124228,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":541,"record_offset":127396,"parent_record_offset":127356,"ancestor_record_offset":124228,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":541,"record_offset":127804,"parent_record_offset":127780,"ancestor_record_offset":124228,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":541,"record_offset":127448,"parent_record_offset":127396,"ancestor_record_offset":124228,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":541,"record_offset":127780,"parent_record_offset":127600,"ancestor_record_offset":124228,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":541,"record_offset":129004,"parent_record_offset":128832,"ancestor_record_offset":124228,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":541,"record_offset":128136,"parent_record_offset":128108,"ancestor_record_offset":124228,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":125852,"parent_record_offset":125824,"ancestor_record_offset":124228,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":125708,"parent_record_offset":125588,"ancestor_record_offset":124228,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":127004,"parent_record_offset":126968,"ancestor_record_offset":124228,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":125624,"parent_record_offset":125588,"ancestor_record_offset":124228,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":125732,"parent_record_offset":125708,"ancestor_record_offset":124228,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":125648,"parent_record_offset":125624,"ancestor_record_offset":124228,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":541,"record_offset":127028,"parent_record_offset":127004,"ancestor_record_offset":124228,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":541,"record_offset":125360,"parent_record_offset":125336,"ancestor_record_offset":124228,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":541,"record_offset":128068,"parent_record_offset":128040,"ancestor_record_offset":124228,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":541,"record_offset":125312,"parent_record_offset":125288,"ancestor_record_offset":124228,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":541,"record_offset":124932,"parent_record_offset":124904,"ancestor_record_offset":124228,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":541,"record_offset":124964,"parent_record_offset":124932,"ancestor_record_offset":124228,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":541,"record_offset":124904,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":541,"record_offset":124764,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":541,"record_offset":125064,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":541,"record_offset":124848,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":541,"record_offset":124584,"parent_record_offset":124548,"ancestor_record_offset":124228,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":541,"record_offset":124608,"parent_record_offset":124584,"ancestor_record_offset":124228,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":541,"record_offset":125288,"parent_record_offset":125180,"ancestor_record_offset":124228,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":125228,"parent_record_offset":125204,"ancestor_record_offset":124228,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":125148,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":125204,"parent_record_offset":125180,"ancestor_record_offset":124228,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":124660,"parent_record_offset":124636,"ancestor_record_offset":124228,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":124636,"parent_record_offset":124608,"ancestor_record_offset":124228,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":124684,"parent_record_offset":124660,"ancestor_record_offset":124228,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":124736,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":541,"record_offset":125180,"parent_record_offset":124684,"ancestor_record_offset":124228,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":125796,"parent_record_offset":125768,"ancestor_record_offset":124228,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":541,"record_offset":125824,"parent_record_offset":125796,"ancestor_record_offset":124228,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":541,"record_offset":125552,"parent_record_offset":125496,"ancestor_record_offset":124228,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":541,"record_offset":125768,"parent_record_offset":125552,"ancestor_record_offset":124228,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":541,"record_offset":126940,"parent_record_offset":126912,"ancestor_record_offset":124228,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":541,"record_offset":125588,"parent_record_offset":125552,"ancestor_record_offset":124228,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":541,"record_offset":126968,"parent_record_offset":126940,"ancestor_record_offset":124228,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":541,"record_offset":125680,"parent_record_offset":125588,"ancestor_record_offset":124228,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00541.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EB6
// 首条位置: {"module":567,"inlinee":32438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":44744,"parent_record_offset":44720,"ancestor_record_offset":44720,"inlinee":13703,"source_line":1350,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1585,"record_offset":44768,"parent_record_offset":44744,"ancestor_record_offset":44720,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EB2
// 首条位置: {"module":567,"inlinee":32434}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1585,"record_offset":69540,"parent_record_offset":69508,"ancestor_record_offset":69508,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1585,"record_offset":69572,"parent_record_offset":69540,"ancestor_record_offset":69508,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1585,"record_offset":69600,"parent_record_offset":69540,"ancestor_record_offset":69508,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EB4
// 首条位置: {"module":567,"inlinee":32436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12343
// 首条位置: {"module":1632,"inlinee":74563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1632,"record_offset":1740,"parent_record_offset":1696,"ancestor_record_offset":1696,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":1920,"parent_record_offset":1696,"ancestor_record_offset":1696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::dispatch_1d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":1892,"parent_record_offset":1696,"ancestor_record_offset":1696,"inlinee":29688,"source_line":630,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1632,"record_offset":1864,"parent_record_offset":1696,"ancestor_record_offset":1696,"inlinee":74564,"source_line":615,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1632,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":1696,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1632,"record_offset":2152,"parent_record_offset":2064,"ancestor_record_offset":1696,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1696,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1696,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1632,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1696,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1632,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1696,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1632,"record_offset":1992,"parent_record_offset":1968,"ancestor_record_offset":1696,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1632,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":1696,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":1968,"parent_record_offset":1944,"ancestor_record_offset":1696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1632,"record_offset":1804,"parent_record_offset":1740,"ancestor_record_offset":1696,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1632,"record_offset":1772,"parent_record_offset":1740,"ancestor_record_offset":1696,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1696,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01632.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_tiles_gravity.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants>", "constructor": "core::option::Option", "arguments": ["country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_tiles_gravity.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/roof_tiles_gravity.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: roof_tiles_gravity.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::compute_pass_builder::impl$3::uniform","relationship":"pdb_inline_direct","evidence":{"module":541,"record_offset":130012,"parent_record_offset":124228,"ancestor_record_offset":124228,"inlinee":12553,"source_line":682,"archive":"_pdb/B/modules/00541.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [Shader 映射] {"shader_id":"8ddcce52d5683171fcc7d918","relative_path":"_roof_tiles_gravity.cs.hlsl_cs_main_0.bin","sha256":"706c3651a87f2c3fdcaa3950e866711e958346ba19482c14430addc7b741c5c0","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"roof_tiles_gravity","prefix_candidates":["roof_tiles_gravity"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _roof_tiles_gravity.cs.hlsl_cs_main_0.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"cs_main","interface_ids":[3]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[64,1,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":22,"name":"frame_constants_storage_buffer","storage_class":"Uniform","type":{"id":71,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,19]},"decorations":[],"pointee":{"id":19,"name":"type.StructuredBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[29]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":29,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"NonWritable","operands":[]}]}]}},"descriptor_set":1,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"all_members_declared_nonwritable","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[1]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":24,"name":"push_constants","storage_class":"PushConstant","type":{"id":72,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[9,23]},"decorations":[],"pointee":{"id":23,"name":"type.ConstantBuffer.PushConstants","record":{"opcode":"OpTypeStruct","operands":[38,38,36,36,36]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"first_instance","type_id":38,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"instance_count","type_id":38,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"wall_height_delta","type_id":36,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":3,"name":"roof_pivot_y","type_id":36,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":4,"name":"animation_t","type_id":36,"decorations":[{"decoration":"Offset","operands":[16]}]}]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":"unknown","declared_direction":"push_constant_input","decorations":[],"access_qualifiers":["Block"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":27,"name":"__alias_instanced_data","storage_class":"Uniform","type":{"id":73,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,25]},"decorations":[],"pointee":{"id":25,"name":"type.RWStructuredBuffer.RoofShingle","record":{"opcode":"OpTypeStruct","operands":[30]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"unknown","type_id":30,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":75,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,74]},"decorations":[],"pointee":{"id":74,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[38,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5161232,"caller_aliases":{"alias_group_rva":5161232,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5161905,"position_among_mapped_render_targets":24,"mapped_render_target_count":59,"previous_callsite":5161894,"next_callsite":5161916,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/leaf_particles.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/stairs.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_tiles_gravity.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>,bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::signifier_replay_active::ReplayModeElapsedTime>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::history::signifier_replay_active::SignifierReplayIndex> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::ivy::ivy_leaf_animation::IvyLeafIsGrowing>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
// 信息 ID: 86ea7dbe5fb1bb2bd6b1aafc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec3b1 -> 0xa29b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0
// 信息 ID: 85f9dcb9828e713812f740ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>; module=1585, inline_record=44720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0
// 信息 ID: 7cb54d53bb9c8fcc9f819505
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4f12dbb1c2f8707c218c2f1d: enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; module=1585, inline_record=69508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure$0
// 信息 ID: 96db63b81e739bf70dabf9df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4f12dbb1c2f8707c218c2f1d: enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0 <- core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; module=1585, inline_record=68644; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; module=1585, inline_record=69968; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>; module=1585, inline_record=70516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$1
// 信息 ID: e7aa8772db1f895bcc4dbc96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1 <- country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_with<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$1,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchApi> >; module=1632, inline_record=1696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
