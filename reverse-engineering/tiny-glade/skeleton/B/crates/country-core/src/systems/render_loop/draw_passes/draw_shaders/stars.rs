// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a5c75386c6ba3cdb456d4998658d24dbcb62542b4f554e349192492800a349b5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stars.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 502, 1598
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stars::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":81620,"rva":9452688,"va":5378161808}
// 编译实例 RVAs: 0x903c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140b666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stars.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t2501208","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378161808,"raw_hex":"e9582a2600","size":5,"soft_fail":false,"target_va":5380663024,"va":5378161811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":502,"record_offset":81788,"parent_record_offset":81620,"ancestor_record_offset":81620,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stars::startup_stars
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":502,"record_offset":81820,"rva":9452704,"va":5378161824}
// 编译实例 RVAs: 0x903ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3080966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e8fafcd0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378162465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2342975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e83fc02300","size":5,"soft_fail":false,"target_va":5380505152,"va":5378162172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fa8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stars.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10447830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e8d66b9f00","size":5,"soft_fail":false,"target_va":5388609760,"va":5378161925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33184636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e87c5bfa01","size":5,"soft_fail":false,"target_va":5411347040,"va":5378162399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4262032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e890084100","size":5,"soft_fail":false,"target_va":5382424224,"va":5378162187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2191071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e8df6e2100","size":5,"soft_fail":false,"target_va":5380353184,"va":5378162108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4495921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e8319a4400","size":5,"soft_fail":false,"target_va":5382657856,"va":5378161930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4591827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378161824,"raw_hex":"e8d3104600","size":5,"soft_fail":false,"target_va":5382753792,"va":5378161960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":502,"record_offset":82828,"parent_record_offset":81820,"ancestor_record_offset":81820,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":502,"record_offset":82004,"parent_record_offset":81820,"ancestor_record_offset":81820,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":83168,"parent_record_offset":81820,"ancestor_record_offset":81820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":502,"record_offset":82772,"parent_record_offset":81820,"ancestor_record_offset":81820,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":502,"record_offset":82088,"parent_record_offset":81820,"ancestor_record_offset":81820,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":83356,"parent_record_offset":83332,"ancestor_record_offset":81820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":502,"record_offset":83332,"parent_record_offset":83240,"ancestor_record_offset":81820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83012,"parent_record_offset":82984,"ancestor_record_offset":81820,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83096,"parent_record_offset":83072,"ancestor_record_offset":81820,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83268,"parent_record_offset":83240,"ancestor_record_offset":81820,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83240,"parent_record_offset":83216,"ancestor_record_offset":81820,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83072,"parent_record_offset":83040,"ancestor_record_offset":81820,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83040,"parent_record_offset":83012,"ancestor_record_offset":81820,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":502,"record_offset":83216,"parent_record_offset":83192,"ancestor_record_offset":81820,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":502,"record_offset":82796,"parent_record_offset":82772,"ancestor_record_offset":81820,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":502,"record_offset":82984,"parent_record_offset":82948,"ancestor_record_offset":81820,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":502,"record_offset":82028,"parent_record_offset":82004,"ancestor_record_offset":81820,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":502,"record_offset":82916,"parent_record_offset":82888,"ancestor_record_offset":81820,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":502,"record_offset":82948,"parent_record_offset":82916,"ancestor_record_offset":81820,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":502,"record_offset":82860,"parent_record_offset":82828,"ancestor_record_offset":81820,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":502,"record_offset":82888,"parent_record_offset":82860,"ancestor_record_offset":81820,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":502,"record_offset":82600,"parent_record_offset":82572,"ancestor_record_offset":81820,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":502,"record_offset":82660,"parent_record_offset":82628,"ancestor_record_offset":81820,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":502,"record_offset":82052,"parent_record_offset":82028,"ancestor_record_offset":81820,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":502,"record_offset":82460,"parent_record_offset":82436,"ancestor_record_offset":81820,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":83300,"parent_record_offset":83268,"ancestor_record_offset":81820,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":82628,"parent_record_offset":82572,"ancestor_record_offset":81820,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":502,"record_offset":82216,"parent_record_offset":82184,"ancestor_record_offset":81820,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":502,"record_offset":82324,"parent_record_offset":82296,"ancestor_record_offset":81820,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":83192,"parent_record_offset":83168,"ancestor_record_offset":81820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":502,"record_offset":82412,"parent_record_offset":82296,"ancestor_record_offset":81820,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":82488,"parent_record_offset":82460,"ancestor_record_offset":81820,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":82436,"parent_record_offset":82412,"ancestor_record_offset":81820,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":82516,"parent_record_offset":82460,"ancestor_record_offset":81820,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":502,"record_offset":82136,"parent_record_offset":82112,"ancestor_record_offset":81820,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":502,"record_offset":82372,"parent_record_offset":82348,"ancestor_record_offset":81820,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":502,"record_offset":82240,"parent_record_offset":82216,"ancestor_record_offset":81820,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":502,"record_offset":82572,"parent_record_offset":82184,"ancestor_record_offset":81820,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":502,"record_offset":82112,"parent_record_offset":82088,"ancestor_record_offset":81820,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":502,"record_offset":82268,"parent_record_offset":82240,"ancestor_record_offset":81820,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":502,"record_offset":82296,"parent_record_offset":82268,"ancestor_record_offset":81820,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":502,"record_offset":82184,"parent_record_offset":82160,"ancestor_record_offset":81820,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":502,"record_offset":82160,"parent_record_offset":82136,"ancestor_record_offset":81820,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::stars::startup_stars::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11B5A
// 首条位置: {"module":1598,"inlinee":72538}
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
// 完整结构及原始类型记录: stars.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stars.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/stars.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: stars.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":20946368,"caller_aliases":{"alias_group_rva":20946368,"count":1,"display_examples":["bevy_ecs::system::schedule_system::impl$1::run_unsafe"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":20947046,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":21811792,"caller_aliases":{"alias_group_rva":21811792,"count":1,"display_examples":["bevy_ecs::system::system::System::run_without_applying_deferred"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":21812097,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":22026464,"caller_aliases":{"alias_group_rva":22026464,"count":1,"display_examples":["bevy_ecs::system::system::System::run"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":22026769,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stars.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>", "bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >", "bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>", "tuple$<bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::Critter>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::color_library::ColorLibrarySsbo>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stars::impl$0::build
// 信息 ID: 1a736edaae9abd7000051b37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stars::startup_stars
// 信息 ID: 54a61ce27008ea214d28bb22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 89e0a4ce22b9cfb45244adbf: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0x13fa066 -> 0x903ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x14cd381 -> 0x903ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x1501a11 -> 0x903ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::stars::startup_stars::closure$0
// 信息 ID: e7aa52b8b8d6cbb6a6c81527
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::stars::startup_stars", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
