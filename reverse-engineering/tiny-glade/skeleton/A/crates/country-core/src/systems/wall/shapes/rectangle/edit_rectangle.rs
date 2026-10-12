// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/edit_rectangle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 46938c80c90caf9065d222c02db7c6fd717822668d482b2abfdade5df23deafb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 452
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":452,"record_offset":51360,"rva":6403840,"va":5375112960}
// 编译实例 RVAs: 0x61b700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406248b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e89f900000","size":5,"soft_fail":false,"target_va":5375150256,"va":5375113228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1826261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e8d5dd1b00","size":5,"soft_fail":false,"target_va":5376939904,"va":5375113638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4537974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e8763e4500","size":5,"soft_fail":false,"target_va":5379651280,"va":5375113301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f373c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9550717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e87dbb9100","size":5,"soft_fail":false,"target_va":5384664000,"va":5375113278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-3808317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e8c3e3c5ff","size":5,"soft_fail":false,"target_va":5371305232,"va":5375113544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2240304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e8302f2200","size":5,"soft_fail":false,"target_va":5377353776,"va":5375113467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2840845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375112960,"raw_hex":"e80d592b00","size":5,"soft_fail":false,"target_va":5377954064,"va":5375113214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":452,"record_offset":52352,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":51548,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":52844,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":52140,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":452,"record_offset":53108,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":452,"record_offset":52412,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":22074,"source_line":155,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":452,"record_offset":52496,"parent_record_offset":51360,"ancestor_record_offset":51360,"inlinee":24554,"source_line":173,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":452,"record_offset":53036,"parent_record_offset":53004,"ancestor_record_offset":51360,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":53004,"parent_record_offset":52964,"ancestor_record_offset":51360,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":452,"record_offset":52964,"parent_record_offset":52928,"ancestor_record_offset":51360,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":452,"record_offset":52380,"parent_record_offset":52352,"ancestor_record_offset":51360,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":452,"record_offset":52868,"parent_record_offset":52844,"ancestor_record_offset":51360,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":51648,"parent_record_offset":51620,"ancestor_record_offset":51360,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":52928,"parent_record_offset":52844,"ancestor_record_offset":51360,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":51620,"parent_record_offset":51576,"ancestor_record_offset":51360,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":452,"record_offset":52168,"parent_record_offset":52140,"ancestor_record_offset":51360,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":452,"record_offset":51576,"parent_record_offset":51548,"ancestor_record_offset":51360,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":452,"record_offset":52752,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":452,"record_offset":52780,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":452,"record_offset":52228,"parent_record_offset":52168,"ancestor_record_offset":51360,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":452,"record_offset":51712,"parent_record_offset":51648,"ancestor_record_offset":51360,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":52256,"parent_record_offset":52228,"ancestor_record_offset":51360,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":52312,"parent_record_offset":52228,"ancestor_record_offset":51360,"inlinee":8100,"source_line":1485,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":51776,"parent_record_offset":51648,"ancestor_record_offset":51360,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":452,"record_offset":52200,"parent_record_offset":52168,"ancestor_record_offset":51360,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":452,"record_offset":52032,"parent_record_offset":52008,"ancestor_record_offset":51360,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":452,"record_offset":53068,"parent_record_offset":52964,"ancestor_record_offset":51360,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":452,"record_offset":51980,"parent_record_offset":51648,"ancestor_record_offset":51360,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":452,"record_offset":51844,"parent_record_offset":51820,"ancestor_record_offset":51360,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":452,"record_offset":51820,"parent_record_offset":51776,"ancestor_record_offset":51360,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":452,"record_offset":52008,"parent_record_offset":51980,"ancestor_record_offset":51360,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":452,"record_offset":52692,"parent_record_offset":52668,"ancestor_record_offset":51360,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":452,"record_offset":52716,"parent_record_offset":52692,"ancestor_record_offset":51360,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_try_update_state
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":452,"record_offset":52520,"parent_record_offset":52496,"ancestor_record_offset":51360,"inlinee":24555,"source_line":185,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::impl$0::update_wall_state_enclosure::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":452,"record_offset":52640,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":24557,"source_line":174,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":452,"record_offset":52608,"parent_record_offset":52584,"ancestor_record_offset":51360,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":452,"record_offset":52808,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":452,"record_offset":52668,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":452,"record_offset":52556,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00452.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":452,"record_offset":52584,"parent_record_offset":52520,"ancestor_record_offset":51360,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/00452.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::edit_rectangle::edit_rectangle
// 信息 ID: 65690533583fbf203db7cee3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9954fc57ed789375b7c94cd9: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> @ 0xd26761 -> 0x61b700 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::shapes::rectangle::EditRectangleCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)> > @ 0xe7ce63 -> 0x61b700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
