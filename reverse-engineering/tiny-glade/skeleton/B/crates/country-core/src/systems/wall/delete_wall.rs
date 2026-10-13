// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/delete_wall.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 772d6446b9ad81aac7c6f7abe031c2c2e4d7a4a35c050b6c4eb40ca07764a47b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 562
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":562,"record_offset":70992,"rva":11190752,"va":5379899872}
// 编译实例 RVAs: 0xaac1e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5223961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e8e749b0ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5379900292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4819282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e8ae76b6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379900781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140885fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2254437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e89b99ddff","size":5,"soft_fail":false,"target_va":5377646512,"va":5379900944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b76f90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t830449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e8f1ab0c00","size":5,"soft_fail":false,"target_va":5380730768,"va":5379900314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b777f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t832438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e8b6b30c00","size":5,"soft_fail":false,"target_va":5380732912,"va":5379900469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2524000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379899872,"raw_hex":"e860832600","size":5,"soft_fail":false,"target_va":5382424432,"va":5379900427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":562,"record_offset":72256,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":562,"record_offset":71856,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":562,"record_offset":71156,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":562,"record_offset":72600,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":13699,"source_line":407,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":562,"record_offset":72832,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":11831,"source_line":63,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":74640,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":562,"record_offset":72312,"parent_record_offset":70992,"ancestor_record_offset":70992,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":562,"record_offset":73504,"parent_record_offset":73480,"ancestor_record_offset":70992,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":562,"record_offset":73480,"parent_record_offset":73392,"ancestor_record_offset":70992,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":73420,"parent_record_offset":73392,"ancestor_record_offset":70992,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":73392,"parent_record_offset":73368,"ancestor_record_offset":70992,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":562,"record_offset":73368,"parent_record_offset":73344,"ancestor_record_offset":70992,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":71500,"parent_record_offset":71476,"ancestor_record_offset":70992,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":73908,"parent_record_offset":73884,"ancestor_record_offset":70992,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":71476,"parent_record_offset":71244,"ancestor_record_offset":70992,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":562,"record_offset":73272,"parent_record_offset":73248,"ancestor_record_offset":70992,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":562,"record_offset":73320,"parent_record_offset":73296,"ancestor_record_offset":70992,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":562,"record_offset":72996,"parent_record_offset":72968,"ancestor_record_offset":70992,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":562,"record_offset":72280,"parent_record_offset":72256,"ancestor_record_offset":70992,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":562,"record_offset":71220,"parent_record_offset":71180,"ancestor_record_offset":70992,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":562,"record_offset":71244,"parent_record_offset":71220,"ancestor_record_offset":70992,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":562,"record_offset":71880,"parent_record_offset":71856,"ancestor_record_offset":70992,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":562,"record_offset":72216,"parent_record_offset":72192,"ancestor_record_offset":70992,"inlinee":15366,"source_line":90,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":562,"record_offset":71180,"parent_record_offset":71156,"ancestor_record_offset":70992,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":562,"record_offset":71936,"parent_record_offset":71912,"ancestor_record_offset":70992,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":562,"record_offset":71912,"parent_record_offset":71880,"ancestor_record_offset":70992,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":562,"record_offset":72016,"parent_record_offset":71992,"ancestor_record_offset":70992,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":562,"record_offset":72904,"parent_record_offset":72856,"ancestor_record_offset":70992,"inlinee":11087,"source_line":807,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":562,"record_offset":72856,"parent_record_offset":72832,"ancestor_record_offset":70992,"inlinee":11085,"source_line":573,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":562,"record_offset":73600,"parent_record_offset":73572,"ancestor_record_offset":70992,"inlinee":6982,"source_line":358,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":562,"record_offset":73572,"parent_record_offset":72856,"ancestor_record_offset":70992,"inlinee":6980,"source_line":49,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":562,"record_offset":73628,"parent_record_offset":73600,"ancestor_record_offset":70992,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":562,"record_offset":73728,"parent_record_offset":73704,"ancestor_record_offset":70992,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":71432,"parent_record_offset":71404,"ancestor_record_offset":70992,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":71292,"parent_record_offset":71244,"ancestor_record_offset":70992,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":562,"record_offset":73448,"parent_record_offset":73420,"ancestor_record_offset":70992,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":562,"record_offset":72968,"parent_record_offset":72904,"ancestor_record_offset":70992,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":562,"record_offset":73860,"parent_record_offset":73828,"ancestor_record_offset":70992,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":71960,"parent_record_offset":71936,"ancestor_record_offset":70992,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":73168,"parent_record_offset":72904,"ancestor_record_offset":70992,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":73192,"parent_record_offset":73168,"ancestor_record_offset":70992,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":72192,"parent_record_offset":71880,"ancestor_record_offset":70992,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":71992,"parent_record_offset":71912,"ancestor_record_offset":70992,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":73704,"parent_record_offset":73680,"ancestor_record_offset":70992,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":73680,"parent_record_offset":73656,"ancestor_record_offset":70992,"inlinee":11885,"source_line":2240,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$33::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":562,"record_offset":73656,"parent_record_offset":73628,"ancestor_record_offset":70992,"inlinee":11884,"source_line":2374,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":73224,"parent_record_offset":72904,"ancestor_record_offset":70992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":562,"record_offset":71748,"parent_record_offset":71724,"ancestor_record_offset":70992,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":562,"record_offset":72140,"parent_record_offset":72116,"ancestor_record_offset":70992,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":562,"record_offset":73104,"parent_record_offset":72996,"ancestor_record_offset":70992,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":562,"record_offset":73040,"parent_record_offset":72996,"ancestor_record_offset":70992,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":73128,"parent_record_offset":73104,"ancestor_record_offset":70992,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":562,"record_offset":73784,"parent_record_offset":73728,"ancestor_record_offset":70992,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":562,"record_offset":71348,"parent_record_offset":71244,"ancestor_record_offset":70992,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":562,"record_offset":71700,"parent_record_offset":71244,"ancestor_record_offset":70992,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":562,"record_offset":71404,"parent_record_offset":71376,"ancestor_record_offset":70992,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":562,"record_offset":71376,"parent_record_offset":71348,"ancestor_record_offset":70992,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":562,"record_offset":71724,"parent_record_offset":71700,"ancestor_record_offset":70992,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":562,"record_offset":72116,"parent_record_offset":72092,"ancestor_record_offset":70992,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":562,"record_offset":72068,"parent_record_offset":72016,"ancestor_record_offset":70992,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":562,"record_offset":72092,"parent_record_offset":72068,"ancestor_record_offset":70992,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::delete_wall::delete_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/delete_wall.rs
//     evidence={"module":562,"record_offset":73884,"parent_record_offset":73860,"ancestor_record_offset":70992,"inlinee":32238,"source_line":23,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":562,"record_offset":72336,"parent_record_offset":72312,"ancestor_record_offset":70992,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":562,"record_offset":72360,"parent_record_offset":72336,"ancestor_record_offset":70992,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":562,"record_offset":72388,"parent_record_offset":72360,"ancestor_record_offset":70992,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::delete_wall::delete_wall::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7DEE
// 首条位置: {"module":562,"inlinee":32238}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":562,"record_offset":73908,"parent_record_offset":73884,"ancestor_record_offset":73884,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00562.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":562,"record_offset":73932,"parent_record_offset":73908,"ancestor_record_offset":73884,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00562.jsonl.gz"}
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
// 完整结构及原始类型记录: delete_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: delete_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: delete_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall
// 信息 ID: 3e1221cd44e1267c6f23e16a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 1e9ad3b1728ab4991be7dd9c: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> > @ 0x294233 -> 0xaac1e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::system::commands::Commands)> > > @ 0x47c378 -> 0xaac1e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::delete_wall::delete_wall::closure$0
// 信息 ID: 44c12b8626d160bf7a29c620
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::delete_wall::delete_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::delete_wall::delete_wall::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::option::IntoIter<alloc::vec::Vec<bevy_ecs::entity::Entity,alloc::alloc::Global> >,alloc::vec::into_iter::IntoIter<bevy_ecs::entity::Entity,alloc::alloc::Global>,country_core::systems::wall::delete_wall::delete_wall::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::wall::delete_wall::delete_wall; module=562, inline_record=73884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
