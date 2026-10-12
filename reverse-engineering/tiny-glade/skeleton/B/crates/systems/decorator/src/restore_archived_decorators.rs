// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/restore_archived_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 591388c7e53eee7632ed62027937c1e27948d90751fd183c0ba11033f6e7fd84
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: restore_archived_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 460
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
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::restore_archived_decorators::restore_archived_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":460,"record_offset":264,"rva":8485440,"va":5377194560}
// 编译实例 RVAs: 0x817a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005c60
//     owner=unresolved_namespace; count=1
//     target_aliases=33 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8464341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e82bd87eff","size":5,"soft_fail":false,"target_va":5368732768,"va":5377197104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8241138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e80e4082ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5377194861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=3
//     target_aliases=35 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7413692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e844e08eff","size":5,"soft_fail":false,"target_va":5369782624,"va":5377196311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9200
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5566511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8d10fabff","size":5,"soft_fail":false,"target_va":5371630080,"va":5377196586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403c03c0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4555686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e85a7cbaff","size":5,"soft_fail":false,"target_va":5372642240,"va":5377197921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t673989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8c5480a00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377195094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2110060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e86c322000","size":5,"soft_fail":false,"target_va":5379306720,"va":5377196655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2110884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8a4352000","size":5,"soft_fail":false,"target_va":5379306912,"va":5377196023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2114608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e830442000","size":5,"soft_fail":false,"target_va":5379310240,"va":5377195627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b743e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3521113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e859ba3500","size":5,"soft_fail":false,"target_va":5380719584,"va":5377198466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b776d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3536048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8b0f43500","size":5,"soft_fail":false,"target_va":5380732624,"va":5377196571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fbfa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10370386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8523d9e00","size":5,"soft_fail":false,"target_va":5387567008,"va":5377196617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141257900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10746902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e816fca300","size":5,"soft_fail":false,"target_va":5387942144,"va":5377195237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141298110
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11009353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e849fda700","size":5,"soft_fail":false,"target_va":5388206352,"va":5377196994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::restore_backup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t944335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8cf680e00","size":5,"soft_fail":false,"target_va":5378142736,"va":5377198396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3227094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8d63d3100","size":5,"soft_fail":false,"target_va":5380422400,"va":5377195301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_storages
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3227641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8f93f3100","size":5,"soft_fail":false,"target_va":5380422528,"va":5377194882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3228098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8c2413100","size":5,"soft_fail":false,"target_va":5380426544,"va":5377198441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoMoveToStorage::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2110622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e89e342000","size":5,"soft_fail":false,"target_va":5379306544,"va":5377195917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2117907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e813512000","size":5,"soft_fail":false,"target_va":5379313120,"va":5377195208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2117564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8bc4f2000","size":5,"soft_fail":false,"target_va":5379313456,"va":5377195887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::mark_mutable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2115596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e80c482000","size":5,"soft_fail":false,"target_va":5379310256,"va":5377194655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t272093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377194560,"raw_hex":"e8dd260400","size":5,"soft_fail":false,"target_va":5377467744,"va":5377195646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":460,"record_offset":440,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":460,"record_offset":1072,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":460,"record_offset":496,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":460,"record_offset":15880,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/restore_archived_decorators.rs
//     evidence={"module":460,"record_offset":1460,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25711,"source_line":80,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":460,"record_offset":11720,"parent_record_offset":11692,"ancestor_record_offset":264,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":460,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":460,"record_offset":720,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":460,"record_offset":464,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":460,"record_offset":584,"parent_record_offset":532,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":460,"record_offset":620,"parent_record_offset":584,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":460,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":264,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":460,"record_offset":532,"parent_record_offset":496,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":460,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":460,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":264,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":264,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":264,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2376,"parent_record_offset":2340,"ancestor_record_offset":264,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":264,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":264,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":264,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6096,"parent_record_offset":5940,"ancestor_record_offset":264,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":264,"inlinee":9025,"source_line":185,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6052,"parent_record_offset":6024,"ancestor_record_offset":264,"inlinee":9026,"source_line":200,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":264,"inlinee":9023,"source_line":178,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":264,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":264,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":264,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6388,"parent_record_offset":6340,"ancestor_record_offset":264,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6732,"parent_record_offset":6340,"ancestor_record_offset":264,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":460,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":460,"record_offset":9700,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":264,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1152,"parent_record_offset":1112,"ancestor_record_offset":264,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1612,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":460,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":264,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":460,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":664,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":264,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6476,"parent_record_offset":6388,"ancestor_record_offset":264,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6448,"parent_record_offset":6388,"ancestor_record_offset":264,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":2176,"parent_record_offset":2144,"ancestor_record_offset":264,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":2144,"parent_record_offset":2112,"ancestor_record_offset":264,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":460,"record_offset":1704,"parent_record_offset":1640,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":460,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":264,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":1676,"parent_record_offset":1640,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2664,"parent_record_offset":1752,"ancestor_record_offset":264,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":264,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2572,"parent_record_offset":1836,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2724,"parent_record_offset":1728,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":264,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":264,"inlinee":11885,"source_line":2240,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$33::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2800,"parent_record_offset":2748,"ancestor_record_offset":264,"inlinee":11884,"source_line":2374,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":1240,"parent_record_offset":1112,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":460,"record_offset":2468,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":460,"record_offset":15904,"parent_record_offset":15880,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":460,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":264,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":460,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":460,"record_offset":776,"parent_record_offset":620,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":460,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":460,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":460,"record_offset":11748,"parent_record_offset":11720,"ancestor_record_offset":264,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":460,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":264,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":264,"inlinee":9030,"source_line":155,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":264,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$58::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":7780,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":25001,"source_line":277,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":7816,"parent_record_offset":7780,"ancestor_record_offset":264,"inlinee":25003,"source_line":351,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_archivist::impl$0::all_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":460,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":264,"inlinee":25715,"source_line":255,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_archivist::impl$0::all_storages::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":460,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":264,"inlinee":25713,"source_line":272,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::Decorators::decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":460,"record_offset":4688,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":24992,"source_line":64,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::Decorators::insert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":460,"record_offset":5768,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":25716,"source_line":159,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$127::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":460,"record_offset":11692,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":25718,"source_line":1548,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2072,"parent_record_offset":2028,"ancestor_record_offset":264,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2112,"parent_record_offset":2072,"ancestor_record_offset":264,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2220,"parent_record_offset":2028,"ancestor_record_offset":264,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2028,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9116,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2256,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9728,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":264,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2312,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9200,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":460,"record_offset":9852,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":460,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":264,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":8924,"parent_record_offset":8892,"ancestor_record_offset":264,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":8956,"parent_record_offset":8924,"ancestor_record_offset":264,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":4772,"parent_record_offset":4744,"ancestor_record_offset":264,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":264,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":264,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":9584,"parent_record_offset":9416,"ancestor_record_offset":264,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":264,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":264,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":264,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4164,"parent_record_offset":3532,"ancestor_record_offset":264,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":264,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":2412,"parent_record_offset":1968,"ancestor_record_offset":264,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9820,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4892,"parent_record_offset":4828,"ancestor_record_offset":264,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":264,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":264,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9028,"parent_record_offset":8996,"ancestor_record_offset":264,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":8996,"parent_record_offset":8956,"ancestor_record_offset":264,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4800,"parent_record_offset":4772,"ancestor_record_offset":264,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4864,"parent_record_offset":4828,"ancestor_record_offset":264,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9060,"parent_record_offset":9028,"ancestor_record_offset":264,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":264,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9880,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":264,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":264,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9416,"parent_record_offset":9060,"ancestor_record_offset":264,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":460,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":264,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":460,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":264,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":8892,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":7748,"parent_record_offset":5768,"ancestor_record_offset":264,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":264,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":4716,"parent_record_offset":1460,"ancestor_record_offset":264,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":264,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":264,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":460,"record_offset":7692,"parent_record_offset":5768,"ancestor_record_offset":264,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":460,"record_offset":5816,"parent_record_offset":5768,"ancestor_record_offset":264,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":5972,"parent_record_offset":5940,"ancestor_record_offset":264,"inlinee":9021,"source_line":85,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":6292,"parent_record_offset":5940,"ancestor_record_offset":264,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":6220,"parent_record_offset":6168,"ancestor_record_offset":264,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x646F
// 首条位置: {"module":460,"inlinee":25711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1612,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":4632,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$58::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":7780,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":25001,"source_line":277,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":460,"record_offset":4688,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":24992,"source_line":64,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::insert
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":460,"record_offset":5768,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":25716,"source_line":159,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$127::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":460,"record_offset":11692,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":25718,"source_line":1548,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":8892,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":7848,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":4716,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":5096,"parent_record_offset":1460,"ancestor_record_offset":1460,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":460,"record_offset":11720,"parent_record_offset":11692,"ancestor_record_offset":1460,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":460,"record_offset":9632,"parent_record_offset":9608,"ancestor_record_offset":1460,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":460,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":1460,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2280,"parent_record_offset":2256,"ancestor_record_offset":1460,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":1460,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":460,"record_offset":2376,"parent_record_offset":2340,"ancestor_record_offset":1460,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":1460,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":1460,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":1460,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":460,"record_offset":6096,"parent_record_offset":5940,"ancestor_record_offset":1460,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":1460,"inlinee":9025,"source_line":185,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6052,"parent_record_offset":6024,"ancestor_record_offset":1460,"inlinee":9026,"source_line":200,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":1460,"inlinee":9023,"source_line":178,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":1460,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":1460,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":1460,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6388,"parent_record_offset":6340,"ancestor_record_offset":1460,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":460,"record_offset":6732,"parent_record_offset":6340,"ancestor_record_offset":1460,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":460,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":1460,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":460,"record_offset":9700,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1640,"parent_record_offset":1612,"ancestor_record_offset":1460,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1460,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":460,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":1460,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":460,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1460,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":460,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1460,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":1460,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=69
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6476,"parent_record_offset":6388,"ancestor_record_offset":1460,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=45
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":460,"record_offset":6448,"parent_record_offset":6388,"ancestor_record_offset":1460,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":2176,"parent_record_offset":2144,"ancestor_record_offset":1460,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":460,"record_offset":2144,"parent_record_offset":2112,"ancestor_record_offset":1460,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":460,"record_offset":1704,"parent_record_offset":1640,"ancestor_record_offset":1460,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":460,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1460,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":1676,"parent_record_offset":1640,"ancestor_record_offset":1460,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2664,"parent_record_offset":1752,"ancestor_record_offset":1460,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1460,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2572,"parent_record_offset":1836,"ancestor_record_offset":1460,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2724,"parent_record_offset":1728,"ancestor_record_offset":1460,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1460,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":1460,"inlinee":11885,"source_line":2240,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$33::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":460,"record_offset":2800,"parent_record_offset":2748,"ancestor_record_offset":1460,"inlinee":11884,"source_line":2374,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":460,"record_offset":2468,"parent_record_offset":1968,"ancestor_record_offset":1460,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":460,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1460,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":460,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1460,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":460,"record_offset":11748,"parent_record_offset":11720,"ancestor_record_offset":1460,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":460,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":1460,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$26::hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":1460,"inlinee":9030,"source_line":155,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":9656,"parent_record_offset":9632,"ancestor_record_offset":1460,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":460,"record_offset":7816,"parent_record_offset":7780,"ancestor_record_offset":1460,"inlinee":25003,"source_line":351,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_archivist::impl$0::all_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":460,"record_offset":4304,"parent_record_offset":4280,"ancestor_record_offset":1460,"inlinee":25715,"source_line":255,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_archivist::impl$0::all_storages::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":460,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1460,"inlinee":25713,"source_line":272,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2072,"parent_record_offset":2028,"ancestor_record_offset":1460,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2112,"parent_record_offset":2072,"ancestor_record_offset":1460,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2220,"parent_record_offset":2028,"ancestor_record_offset":1460,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":460,"record_offset":2028,"parent_record_offset":1968,"ancestor_record_offset":1460,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9116,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2256,"parent_record_offset":1968,"ancestor_record_offset":1460,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9728,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":1460,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":2312,"parent_record_offset":1968,"ancestor_record_offset":1460,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":460,"record_offset":9200,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":460,"record_offset":9852,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":460,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":1460,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":8924,"parent_record_offset":8892,"ancestor_record_offset":1460,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":8956,"parent_record_offset":8924,"ancestor_record_offset":1460,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":4772,"parent_record_offset":4744,"ancestor_record_offset":1460,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":1460,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":1460,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":9584,"parent_record_offset":9416,"ancestor_record_offset":1460,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1460,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":460,"record_offset":5916,"parent_record_offset":5888,"ancestor_record_offset":1460,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":1460,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4164,"parent_record_offset":3532,"ancestor_record_offset":1460,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":1460,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":2412,"parent_record_offset":1968,"ancestor_record_offset":1460,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9820,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4892,"parent_record_offset":4828,"ancestor_record_offset":1460,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":1460,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":1460,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9028,"parent_record_offset":8996,"ancestor_record_offset":1460,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":8996,"parent_record_offset":8956,"ancestor_record_offset":1460,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4800,"parent_record_offset":4772,"ancestor_record_offset":1460,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4864,"parent_record_offset":4828,"ancestor_record_offset":1460,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9060,"parent_record_offset":9028,"ancestor_record_offset":1460,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":1460,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9880,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1460,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":460,"record_offset":9416,"parent_record_offset":9060,"ancestor_record_offset":1460,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":460,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":1460,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":460,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":1460,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":7748,"parent_record_offset":5768,"ancestor_record_offset":1460,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":1460,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1460,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$58::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":460,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":1460,"inlinee":11860,"source_line":2516,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":460,"record_offset":7692,"parent_record_offset":5768,"ancestor_record_offset":1460,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":460,"record_offset":5816,"parent_record_offset":5768,"ancestor_record_offset":1460,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":5972,"parent_record_offset":5940,"ancestor_record_offset":1460,"inlinee":9021,"source_line":85,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":6292,"parent_record_offset":5940,"ancestor_record_offset":1460,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/00460.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":460,"record_offset":6220,"parent_record_offset":6168,"ancestor_record_offset":1460,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/00460.jsonl.gz"}
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
// 完整结构及原始类型记录: restore_archived_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: restore_archived_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: restore_archived_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>

// ------------------------------------------------------------------
// 函数: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
// 信息 ID: a2f3d18c7d6e6e5599c62376
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::restore_archived_decorators::restore_archived_decorators; module=460, inline_record=1460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::restore_archived_decorators::restore_archived_decorators
// 信息 ID: 35240dad62e247a6ece38ffc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 668723924bc498d8fca5e438: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)> > @ 0x3221eb -> 0x817a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)> > > @ 0x45984d -> 0x817a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
