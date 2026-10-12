// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/edit_rectangle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e3efd4fefdafbc2c70fa235804f595b90d2b9e7d0c179d71d1e2b8f49abe267f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 519
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
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":519,"record_offset":84832,"rva":9958944,"va":5378668064}
// 编译实例 RVAs: 0x97f620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1220268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378668064,"raw_hex":"e8ac9e1200","size":5,"soft_fail":false,"target_va":5379888656,"va":5378668383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1223015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378668064,"raw_hex":"e867a91200","size":5,"soft_fail":false,"target_va":5379891344,"va":5378668324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3214461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378668064,"raw_hex":"e87d0c3100","size":5,"soft_fail":false,"target_va":5381883040,"va":5378668574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4311921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378668064,"raw_hex":"e88f34beff","size":5,"soft_fail":false,"target_va":5374356528,"va":5378668444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3249621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378668064,"raw_hex":"e8d5953100","size":5,"soft_fail":false,"target_va":5381917936,"va":5378668310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":519,"record_offset":86376,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":519,"record_offset":86104,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":519,"record_offset":85020,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":519,"record_offset":86548,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":519,"record_offset":86876,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":519,"record_offset":86436,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":519,"record_offset":86520,"parent_record_offset":84832,"ancestor_record_offset":84832,"inlinee":26544,"source_line":196,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":85384,"parent_record_offset":85360,"ancestor_record_offset":84832,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":85432,"parent_record_offset":85408,"ancestor_record_offset":84832,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":519,"record_offset":85408,"parent_record_offset":85384,"ancestor_record_offset":84832,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":86712,"parent_record_offset":86672,"ancestor_record_offset":84832,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":85360,"parent_record_offset":85332,"ancestor_record_offset":84832,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":85332,"parent_record_offset":85304,"ancestor_record_offset":84832,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":86672,"parent_record_offset":86636,"ancestor_record_offset":84832,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":519,"record_offset":85304,"parent_record_offset":85168,"ancestor_record_offset":84832,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":519,"record_offset":86404,"parent_record_offset":86376,"ancestor_record_offset":84832,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":519,"record_offset":86572,"parent_record_offset":86548,"ancestor_record_offset":84832,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":519,"record_offset":87604,"parent_record_offset":87580,"ancestor_record_offset":84832,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":519,"record_offset":86636,"parent_record_offset":86572,"ancestor_record_offset":84832,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":519,"record_offset":85124,"parent_record_offset":85064,"ancestor_record_offset":84832,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":519,"record_offset":85168,"parent_record_offset":85124,"ancestor_record_offset":84832,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":519,"record_offset":86148,"parent_record_offset":86104,"ancestor_record_offset":84832,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":519,"record_offset":85064,"parent_record_offset":85020,"ancestor_record_offset":84832,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":519,"record_offset":86192,"parent_record_offset":86148,"ancestor_record_offset":84832,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":519,"record_offset":85752,"parent_record_offset":85720,"ancestor_record_offset":84832,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":519,"record_offset":85248,"parent_record_offset":85168,"ancestor_record_offset":84832,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":86236,"parent_record_offset":86192,"ancestor_record_offset":84832,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":86308,"parent_record_offset":86192,"ancestor_record_offset":84832,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":519,"record_offset":86340,"parent_record_offset":86148,"ancestor_record_offset":84832,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":519,"record_offset":85996,"parent_record_offset":85972,"ancestor_record_offset":84832,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":519,"record_offset":86832,"parent_record_offset":86672,"ancestor_record_offset":84832,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":519,"record_offset":85656,"parent_record_offset":85168,"ancestor_record_offset":84832,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":519,"record_offset":85944,"parent_record_offset":85168,"ancestor_record_offset":84832,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":519,"record_offset":85720,"parent_record_offset":85688,"ancestor_record_offset":84832,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":519,"record_offset":85688,"parent_record_offset":85656,"ancestor_record_offset":84832,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00519.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":519,"record_offset":85972,"parent_record_offset":85944,"ancestor_record_offset":84832,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00519.jsonl.gz"}
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
// 完整结构及原始类型记录: edit_rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: edit_rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: edit_rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle
// 信息 ID: b4d881d73e47323d9b56d3c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] df2e290e54dfcb05e1cf8def: void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > @ 0xabd5f0 -> 0x97f620 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > > @ 0x14c2b42 -> 0x97f620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
