// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/tree-tool/src/replay_tree.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 31aa53ab45989214263bd368e95540d01e4be08efedfa823f770cee90010decb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_tree.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 2454
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::replay_tree::replay_tree
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2454,"record_offset":260,"rva":29111536,"va":5397820656}
// 编译实例 RVAs: 0x1bc34f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14973350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e85a861bff","size":5,"soft_fail":false,"target_va":5382847888,"va":5397821233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26074825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e8372172fe","size":5,"soft_fail":false,"target_va":5371747712,"va":5397822532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22741285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e8dbfea4fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5397822784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22201382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e8da3badfe","size":5,"soft_fail":false,"target_va":5375621344,"va":5397822721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20126930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e82ee3ccfe","size":5,"soft_fail":false,"target_va":5377695408,"va":5397822333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140949170
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19375067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e8255cd8fe","size":5,"soft_fail":false,"target_va":5378445680,"va":5397820742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17202401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e81f83f9fe","size":5,"soft_fail":false,"target_va":5380620352,"va":5397822748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17201631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e82186f9fe","size":5,"soft_fail":false,"target_va":5380620960,"va":5397822586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17198992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e87090f9fe","size":5,"soft_fail":false,"target_va":5380623680,"va":5397822667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17198190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e89293f9fe","size":5,"soft_fail":false,"target_va":5380624320,"va":5397822505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17198218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e87693f9fe","size":5,"soft_fail":false,"target_va":5380624400,"va":5397822613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_tree.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14024726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e81600d600","size":5,"soft_fail":false,"target_va":5411847184,"va":5397822453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-23464624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e850f599fe","size":5,"soft_fail":false,"target_va":5374356528,"va":5397821147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13718371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e86353d100","size":5,"soft_fail":false,"target_va":5411541225,"va":5397822849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18684642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397820656,"raw_hex":"e81ee5e2fe","size":5,"soft_fail":false,"target_va":5379136128,"va":5397820765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":596,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":6232,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2454,"record_offset":3908,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2454,"record_offset":428,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":2454,"record_offset":984,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2454,"record_offset":3588,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2454,"record_offset":780,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2454,"record_offset":4040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2454,"record_offset":512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2454,"record_offset":3796,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::tree_edit::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/tree_edit.rs
//     evidence={"module":2454,"record_offset":1148,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":97481,"source_line":12,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2454,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2454,"record_offset":1624,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":260,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":1568,"parent_record_offset":1544,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":260,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":260,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2454,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2454,"record_offset":4152,"parent_record_offset":4116,"ancestor_record_offset":260,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2454,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":260,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":636,"parent_record_offset":596,"ancestor_record_offset":260,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":6280,"parent_record_offset":6232,"ancestor_record_offset":260,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":1100,"parent_record_offset":1068,"ancestor_record_offset":260,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2454,"record_offset":6496,"parent_record_offset":6232,"ancestor_record_offset":260,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2454,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":260,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2454,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":260,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2454,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":260,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2454,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":260,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2454,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":260,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2454,"record_offset":1068,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2454,"record_offset":3868,"parent_record_offset":3824,"ancestor_record_offset":260,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2454,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":260,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2454,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2454,"record_offset":560,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2454,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2454,"record_offset":4184,"parent_record_offset":4152,"ancestor_record_offset":260,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2454,"record_offset":3980,"parent_record_offset":3908,"ancestor_record_offset":260,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2454,"record_offset":860,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2454,"record_offset":4004,"parent_record_offset":3980,"ancestor_record_offset":260,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2454,"record_offset":748,"parent_record_offset":596,"ancestor_record_offset":260,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2454,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":260,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":2454,"record_offset":536,"parent_record_offset":512,"ancestor_record_offset":260,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2454,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":260,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2454,"record_offset":816,"parent_record_offset":780,"ancestor_record_offset":260,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2454,"record_offset":892,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2454,"record_offset":4212,"parent_record_offset":4152,"ancestor_record_offset":260,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2454,"record_offset":4244,"parent_record_offset":4116,"ancestor_record_offset":260,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/02454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$101::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":2454,"record_offset":1188,"parent_record_offset":1148,"ancestor_record_offset":260,"inlinee":25047,"source_line":17,"archive":"_pdb/B/modules/02454.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_tree.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_tree.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_tree.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_tree::replay_tree::replay_tree
// 信息 ID: bd9d8e91ba435965e8cac6a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9bee7888a891991540c06441: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0xe9bb30 -> 0x1bc34f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1bcca12 -> 0x1bc34f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
