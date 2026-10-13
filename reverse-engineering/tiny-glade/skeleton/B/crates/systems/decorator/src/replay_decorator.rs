// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/replay_decorator.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1734f11edf5d6cb7410705e13646147324f7b6b7693770326c3a780dc499dd23
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_decorator.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 452
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
// 函数符号: system_decorator::replay_decorator::replay_decorator
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":264,"rva":8440016,"va":5377149136}
// 编译实例 RVAs: 0x80c8d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9200
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5519260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e864c8abff","size":5,"soft_fail":false,"target_va":5371630080,"va":5377149335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5402393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8e790adff","size":5,"soft_fail":false,"target_va":5371747712,"va":5377150100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2068850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e88e6ee0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377150349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1528950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e88aabe8ff","size":5,"soft_fail":false,"target_va":5375621344,"va":5377150289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t545489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8d1520800","size":5,"soft_fail":false,"target_va":5377695408,"va":5377149914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1296481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e861c81300","size":5,"soft_fail":false,"target_va":5378445680,"va":5377149194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3470031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8cff23400","size":5,"soft_fail":false,"target_va":5380620352,"va":5377150316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3470801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8d1f53400","size":5,"soft_fail":false,"target_va":5380620960,"va":5377150154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3473440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e820003500","size":5,"soft_fail":false,"target_va":5380623680,"va":5377150235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8b9033500","size":5,"soft_fail":false,"target_va":5380623728,"va":5377149362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e842033500","size":5,"soft_fail":false,"target_va":5380624320,"va":5377150073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e826033500","size":5,"soft_fail":false,"target_va":5380624400,"va":5377150181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_decorator.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34697158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e8c66f1102","size":5,"soft_fail":false,"target_va":5411847184,"va":5377150021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3625876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e894533700","size":5,"soft_fail":false,"target_va":5380775408,"va":5377149527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34390828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e82cc30c02","size":5,"soft_fail":false,"target_va":5411541225,"va":5377150392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1986906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377149136,"raw_hex":"e85a511e00","size":5,"soft_fail":false,"target_va":5379136128,"va":5377149217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":1444,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":1780,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":4572,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":452,"record_offset":2248,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":452,"record_offset":444,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":1752,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":452,"record_offset":1556,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":452,"record_offset":500,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":1256,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":452,"record_offset":1472,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":452,"record_offset":2136,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":452,"record_offset":668,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25603,"source_line":302,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":452,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":452,"record_offset":5100,"parent_record_offset":5012,"ancestor_record_offset":264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":452,"record_offset":1316,"parent_record_offset":1280,"ancestor_record_offset":264,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":452,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":264,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":1812,"parent_record_offset":1780,"ancestor_record_offset":264,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":4620,"parent_record_offset":4572,"ancestor_record_offset":264,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":4836,"parent_record_offset":4572,"ancestor_record_offset":264,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":452,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":264,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":452,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":264,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":452,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":452,"record_offset":1052,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":452,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":264,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::filter_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":452,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":264,"inlinee":10097,"source_line":79,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":452,"record_offset":840,"parent_record_offset":812,"ancestor_record_offset":264,"inlinee":10095,"source_line":155,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":452,"record_offset":812,"parent_record_offset":784,"ancestor_record_offset":264,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":452,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":264,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":452,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":264,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":452,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":264,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":452,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":452,"record_offset":2208,"parent_record_offset":2164,"ancestor_record_offset":264,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":452,"record_offset":1192,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":452,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":452,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":264,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":264,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":452,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":452,"record_offset":992,"parent_record_offset":968,"ancestor_record_offset":264,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":1348,"parent_record_offset":1316,"ancestor_record_offset":264,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":2320,"parent_record_offset":2248,"ancestor_record_offset":264,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":968,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":452,"record_offset":940,"parent_record_offset":896,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":548,"parent_record_offset":524,"ancestor_record_offset":264,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":576,"parent_record_offset":524,"ancestor_record_offset":264,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":452,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":264,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":452,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":264,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":264,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":264,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":264,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":452,"record_offset":1660,"parent_record_offset":1584,"ancestor_record_offset":264,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":452,"record_offset":1376,"parent_record_offset":1316,"ancestor_record_offset":264,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":452,"record_offset":1408,"parent_record_offset":1280,"ancestor_record_offset":264,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":452,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":264,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":452,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":264,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":452,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":264,"inlinee":6932,"source_line":1307,"archive":"_pdb/B/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":452,"record_offset":700,"parent_record_offset":668,"ancestor_record_offset":264,"inlinee":13716,"source_line":1117,"archive":"_pdb/B/modules/00452.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_decorator.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_decorator.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_decorator.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>

// ------------------------------------------------------------------
// 函数: system_decorator::replay_decorator::replay_decorator
// 信息 ID: a98305cff5397ab0901d28d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ec1197e5251cda0eee2cfcfd: void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)> > @ 0x321bd9 -> 0x80c8d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)> > > @ 0x46c3cb -> 0x80c8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
