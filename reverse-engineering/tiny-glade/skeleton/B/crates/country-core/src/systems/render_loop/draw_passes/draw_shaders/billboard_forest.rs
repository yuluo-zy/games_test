// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 15458ffd5114c44900c6cbbdb84b809a60e92cd5c4ca9779850b4ca6b5120d84
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: billboard_forest.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 499, 570
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
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":28416,"rva":11436144,"va":5380145264}
// 编译实例 RVAs: 0xae8070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140b68050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t524248","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5380145264,"raw_hex":"e9d8ff0700","size":5,"soft_fail":false,"target_va":5380669520,"va":5380145267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":570,"record_offset":28596,"parent_record_offset":28416,"ancestor_record_offset":28416,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":28628,"rva":11436160,"va":5380145280}
// 编译实例 RVAs: 0xae8080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5064422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e81ab9b2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380145921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b3fe40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t359512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8587c0500","size":5,"soft_fail":false,"target_va":5380505152,"va":5380145635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fa8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: billboard_forest.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8464374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8f6278100","size":5,"soft_fail":false,"target_va":5388609760,"va":5380145381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31201187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8a317dc01","size":5,"soft_fail":false,"target_va":5411347040,"va":5380145848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2278569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8a9c42200","size":5,"soft_fail":false,"target_va":5382424224,"va":5380145650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mesh_into_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t207608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8f82a0300","size":5,"soft_fail":false,"target_va":5380353184,"va":5380145571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2512465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e851562600","size":5,"soft_fail":false,"target_va":5382657856,"va":5380145386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2608371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380145280,"raw_hex":"e8f3cc2700","size":5,"soft_fail":false,"target_va":5382753792,"va":5380145416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::spawn
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":570,"record_offset":29660,"parent_record_offset":28628,"ancestor_record_offset":28628,"inlinee":9432,"source_line":372,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28836,"parent_record_offset":28628,"ancestor_record_offset":28628,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":30000,"parent_record_offset":28628,"ancestor_record_offset":28628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":570,"record_offset":29604,"parent_record_offset":28628,"ancestor_record_offset":28628,"inlinee":25911,"source_line":53,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":570,"record_offset":28920,"parent_record_offset":28628,"ancestor_record_offset":28628,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30188,"parent_record_offset":30164,"ancestor_record_offset":28628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":30164,"parent_record_offset":30072,"ancestor_record_offset":28628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":29844,"parent_record_offset":29816,"ancestor_record_offset":28628,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":29928,"parent_record_offset":29904,"ancestor_record_offset":28628,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":30100,"parent_record_offset":30072,"ancestor_record_offset":28628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":30072,"parent_record_offset":30048,"ancestor_record_offset":28628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":29904,"parent_record_offset":29872,"ancestor_record_offset":28628,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":29872,"parent_record_offset":29844,"ancestor_record_offset":28628,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":30048,"parent_record_offset":30024,"ancestor_record_offset":28628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":29628,"parent_record_offset":29604,"ancestor_record_offset":28628,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29816,"parent_record_offset":29780,"ancestor_record_offset":28628,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":28628,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":570,"record_offset":29748,"parent_record_offset":29720,"ancestor_record_offset":28628,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":570,"record_offset":29780,"parent_record_offset":29748,"ancestor_record_offset":28628,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":570,"record_offset":29692,"parent_record_offset":29660,"ancestor_record_offset":28628,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":570,"record_offset":29720,"parent_record_offset":29692,"ancestor_record_offset":28628,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":28628,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":29492,"parent_record_offset":29460,"ancestor_record_offset":28628,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":570,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":28628,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":29292,"parent_record_offset":29268,"ancestor_record_offset":28628,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":30132,"parent_record_offset":30100,"ancestor_record_offset":28628,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":29460,"parent_record_offset":29404,"ancestor_record_offset":28628,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":570,"record_offset":29048,"parent_record_offset":29016,"ancestor_record_offset":28628,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":570,"record_offset":29156,"parent_record_offset":29128,"ancestor_record_offset":28628,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":30024,"parent_record_offset":30000,"ancestor_record_offset":28628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":29244,"parent_record_offset":29128,"ancestor_record_offset":28628,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":29320,"parent_record_offset":29292,"ancestor_record_offset":28628,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":29268,"parent_record_offset":29244,"ancestor_record_offset":28628,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":29348,"parent_record_offset":29292,"ancestor_record_offset":28628,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":28968,"parent_record_offset":28944,"ancestor_record_offset":28628,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":570,"record_offset":29204,"parent_record_offset":29180,"ancestor_record_offset":28628,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":570,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":28628,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":29404,"parent_record_offset":29016,"ancestor_record_offset":28628,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":28944,"parent_record_offset":28920,"ancestor_record_offset":28628,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":570,"record_offset":29100,"parent_record_offset":29072,"ancestor_record_offset":28628,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":570,"record_offset":29128,"parent_record_offset":29100,"ancestor_record_offset":28628,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":29016,"parent_record_offset":28992,"ancestor_record_offset":28628,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28628,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6DC3
// 首条位置: {"module":499,"inlinee":28099}
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
// 完整结构及原始类型记录: billboard_forest.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,feedback_reporter::FeedbackReporterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::menu_background::MenuBackgroundPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::paths::PathPlugin,bevy_app::plugin::sealed::PluginMarker,system_water::WaterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::garden::GardenPlugin,bevy_app::plugin::sealed::PluginMarker,system_tree::TreePlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::prefab::PrefabPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::photo_mode::PhotoModePlugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "feedback_reporter::FeedbackReporterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::feedback_report_resources::FeedbackReportBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::menu_background::MenuBackgroundPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::paths::PathPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_water::WaterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::garden::GardenPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_tree::TreePlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::prefab::PrefabPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::vermeer::VermeerPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::clouds::CloudPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::stars::StarsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::BillboardForest", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::draw_passes::logo_overlay::LogoOverlayPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::photo_mode::PhotoModePlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: billboard_forest.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/draw_passes/draw_shaders/billboard_forest.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: billboard_forest.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// [Shader 映射] {"shader_id":"a25aad7e1226eaff56576921","relative_path":"_billboard_forest.raster.hlsl_b903e43ffb3da915.bin","sha256":"e4897d894dd19632f65ff8851f29cc575b6864c26bc07d44874b73992cb85dbd","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"billboard_forest","prefix_candidates":["billboard_forest"]},"actual_binding":"unknown"}
// [Shader 映射] {"shader_id":"10e320029e741f1a66e12282","relative_path":"_billboard_forest.raster.hlsl_f0adff76827e73ff.bin","sha256":"1bfc0935a5deca75f558eea4d60d3527d99f90dca4b889a6c6a159fca0fd0420","status":"filename_family_candidate_only; exact Pass binding unknown","basis":{"file_stem":"billboard_forest","prefix_candidates":["billboard_forest"]},"actual_binding":"unknown"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _billboard_forest.raster.hlsl_f0adff76827e73ff.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"frame_constants","storage_class":"Uniform","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,38]},"decorations":[],"pointee":{"id":38,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[67,67,67,67,67,67,67,67,50,48,48,50,58,50,48,52,48,52,50,52,50,48,50,50,50,67,43]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":67,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":50,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":48,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":48,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":50,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":58,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":50,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":48,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":52,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":48,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":52,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":50,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":52,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":50,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":48,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":50,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":50,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":50,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":67,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":43,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"$Globals","storage_class":"Uniform","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[50,58,58,58,58,58]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":50,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"framebuffer_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"cloud1_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"cloud2_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"cloud3_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"cloud4_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,67]},"decorations":[],"pointee":{"id":67,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C3","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C4","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":33,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":52,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,32]},"decorations":[],"pointee":{"id":32,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":35,"name":"cloud1","storage_class":"UniformConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"cloud2","storage_class":"UniformConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":37,"name":"cloud3","storage_class":"UniformConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":38,"name":"cloud4","storage_class":"UniformConstant","type":{"id":53,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,34]},"decorations":[],"pointee":{"id":34,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[44,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":54,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,45]},"decorations":[],"pointee":{"id":45,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[44,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":55,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,46]},"decorations":[],"pointee":{"id":46,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [Shader 声明接口；Pass 实际绑定见映射状态] _billboard_forest.raster.hlsl_b903e43ffb3da915.bin
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Vertex","function_id":2,"entry_name":"vs_main","interface_ids":[3,4,5,6,7,8,9,10]}],"execution_modes":[]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":40,"name":"frame_constants","storage_class":"Uniform","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,38]},"decorations":[],"pointee":{"id":38,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[67,67,67,67,67,67,67,67,50,48,48,50,58,50,48,52,48,52,50,52,50,48,50,50,50,67,43]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":67,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":67,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":67,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":50,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":48,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":48,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":50,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":58,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":50,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":48,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":52,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":48,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":52,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":50,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":52,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":50,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":48,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":50,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":50,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":50,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":67,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":43,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"$Globals","storage_class":"Uniform","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,41]},"decorations":[],"pointee":{"id":41,"name":"type.$Globals","record":{"opcode":"OpTypeStruct","operands":[50,58,58,58,58,58]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"current_view_index","type_id":50,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"framebuffer_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[4]}]},{"index":2,"name":"cloud1_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":3,"name":"cloud2_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[24]}]},{"index":4,"name":"cloud3_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":5,"name":"cloud4_dims","type_id":58,"decorations":[{"decoration":"Offset","operands":[40]}]}]}},"descriptor_set":0,"binding":13,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[13]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"unknown","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":42,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[42]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"unknown","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":43,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[43]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"unknown","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,67]},"decorations":[],"pointee":{"id":67,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,4]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":0,"declared_direction":"stage_output","decorations":[{"decoration":"BuiltIn","operands":[0]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"out.var.C0","storage_class":"Output","type":{"id":93,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,49]},"decorations":[],"pointee":{"id":49,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.C1","storage_class":"Output","type":{"id":94,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,50]},"decorations":[],"pointee":{"id":50,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":8,"name":"out.var.C2","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":2,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[2]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":9,"name":"out.var.C3","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":10,"name":"out.var.C4","storage_class":"Output","type":{"id":95,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,52]},"decorations":[],"pointee":{"id":52,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[48,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"Fragment","function_id":2,"entry_name":"ps_main","interface_ids":[3,4,5,6,7]}],"execution_modes":[{"entry_id":2,"mode":"OriginUpperLeft","operands":[],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":36,"name":"sampler_lnc","storage_class":"UniformConstant","type":{"id":84,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,35]},"decorations":[],"pointee":{"id":35,"name":"type.sampler","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":39,"name":"frame_constants","storage_class":"Uniform","type":{"id":86,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,37]},"decorations":[],"pointee":{"id":37,"name":"type.ConstantBuffer.FrameConstants","record":{"opcode":"OpTypeStruct","operands":[64,64,64,64,64,64,64,64,55,53,53,55,61,55,53,56,53,56,55,56,55,53,55,55,55,64,48]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":64,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":64,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":64,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":64,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"frame_index","type_id":55,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":53,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":53,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":55,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"inner_rendering_resolution","type_id":61,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":55,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":53,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":56,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":53,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":56,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":55,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":56,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":55,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"global_mip_bias","type_id":53,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"highlighted_transient_stair_edge_id","type_id":55,"decorations":[{"decoration":"Offset","operands":[212]}]},{"index":23,"name":"pad1","type_id":55,"decorations":[{"decoration":"Offset","operands":[216]}]},{"index":24,"name":"pad2","type_id":55,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":25,"name":"debug_tweakers","type_id":64,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":26,"name":"view_constants","type_id":48,"decorations":[{"decoration":"Offset","operands":[240]}]}]}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[1]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"cloud1","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":14,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[14]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":42,"name":"cloud2","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":15,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[15]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":43,"name":"cloud3","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":16,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[16]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":44,"name":"cloud4","storage_class":"UniformConstant","type":{"id":87,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,40]},"decorations":[],"pointee":{"id":40,"name":"type.2d.image","record":{"opcode":"OpTypeImage","operands":[53,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":0,"binding":17,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[17]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":46,"name":"color_library","storage_class":"Uniform","type":{"id":88,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,45]},"decorations":[],"pointee":{"id":45,"name":"type.ConstantBuffer.ColorLibrary","record":{"opcode":"OpTypeStruct","operands":[64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":64,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":64,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":64,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":64,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":64,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":64,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":64,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":64,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":64,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":64,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":64,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":64,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":64,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":64,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":64,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":64,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":64,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":64,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":64,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":0,"binding":18,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[0]},{"decoration":"Binding","operands":[18]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":3,"name":"in.var.C0","storage_class":"Input","type":{"id":89,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,54]},"decorations":[],"pointee":{"id":54,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":4,"name":"in.var.C1","storage_class":"Input","type":{"id":90,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,55]},"decorations":[],"pointee":{"id":55,"name":"unknown","record":{"opcode":"OpTypeInt","operands":[32,0]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":1,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Flat","operands":[]},{"decoration":"Location","operands":[1]}],"access_qualifiers":["Flat","Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":5,"name":"in.var.C3","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,56]},"decorations":[],"pointee":{"id":56,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":3,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[3]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":6,"name":"in.var.C4","storage_class":"Input","type":{"id":91,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,56]},"decorations":[],"pointee":{"id":56,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[53,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":4,"builtin":"unknown","declared_direction":"stage_input","decorations":[{"decoration":"Location","operands":[4]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":7,"name":"out.var.SV_TARGET0","storage_class":"Output","type":{"id":92,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[3,61]},"decorations":[],"pointee":{"id":61,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[55,2]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":0,"builtin":"unknown","declared_direction":"stage_output","decorations":[{"decoration":"Location","operands":[0]}],"access_qualifiers":["Location"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":9334512,"caller_aliases":{"alias_group_rva":9334512,"count":1,"display_examples":["bevy_ecs::system::schedule_system::impl$1::run_unsafe"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":9335190,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":22012144,"caller_aliases":{"alias_group_rva":22012144,"count":1,"display_examples":["bevy_ecs::system::system::System::run_without_applying_deferred"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":22012449,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":22043952,"caller_aliases":{"alias_group_rva":22043952,"count":1,"display_examples":["bevy_ecs::system::system::System::run"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":22044257,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: billboard_forest.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::impl$0::build
// 信息 ID: 3f9d6139748807c3a4cd6398
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest
// 信息 ID: 4acd4d60a8f8f81322e3db14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 89e0a4ce22b9cfb45244adbf: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > @ 0x8e7196 -> 0xae8080 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x14fe221 -> 0xae8080 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> > > @ 0x1505e61 -> 0xae8080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest::closure$0
// 信息 ID: fd6379a9643fc7cabaca46c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::draw_passes::draw_shaders::billboard_forest::startup_billboard_forest", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
