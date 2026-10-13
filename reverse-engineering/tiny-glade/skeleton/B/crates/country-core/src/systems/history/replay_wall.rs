// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/history/replay_wall.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 63a5acf58ac3c60edf1aabe491ee98a6c3ef4e0d80b68adaeddcf7479c1caedf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: replay_wall.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 513, 564, 572, 1602, 1610
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":30772,"rva":9774448,"va":5378483568}
// 编译实例 RVAs: 0x952570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3402229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e80b16ccff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378483712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3402602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e89614ccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378484101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1407356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e87c791500","size":5,"soft_fail":false,"target_va":5379891344,"va":5378483983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11617294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e80e44b100","size":5,"soft_fail":false,"target_va":5390101056,"va":5378483757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33363361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e8a115fd01","size":5,"soft_fail":false,"target_va":5411847168,"va":5378483802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33056936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378483568,"raw_hex":"e8a868f801","size":5,"soft_fail":false,"target_va":5411540963,"va":5378484022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":513,"record_offset":32408,"parent_record_offset":30772,"ancestor_record_offset":30772,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":513,"record_offset":30928,"parent_record_offset":30772,"ancestor_record_offset":30772,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":31432,"parent_record_offset":31408,"ancestor_record_offset":30772,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":31456,"parent_record_offset":31432,"ancestor_record_offset":30772,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":32356,"parent_record_offset":32332,"ancestor_record_offset":30772,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":31408,"parent_record_offset":31232,"ancestor_record_offset":30772,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":32332,"parent_record_offset":32244,"ancestor_record_offset":30772,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":31172,"parent_record_offset":31144,"ancestor_record_offset":30772,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":32272,"parent_record_offset":32244,"ancestor_record_offset":30772,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":30772,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":31232,"parent_record_offset":31200,"ancestor_record_offset":30772,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":31200,"parent_record_offset":31172,"ancestor_record_offset":30772,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":30772,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":31268,"parent_record_offset":31232,"ancestor_record_offset":30772,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":31108,"parent_record_offset":31076,"ancestor_record_offset":30772,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":513,"record_offset":31076,"parent_record_offset":31008,"ancestor_record_offset":30772,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":31048,"parent_record_offset":31008,"ancestor_record_offset":30772,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":31144,"parent_record_offset":31108,"ancestor_record_offset":30772,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":30976,"parent_record_offset":30928,"ancestor_record_offset":30772,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":31008,"parent_record_offset":30976,"ancestor_record_offset":30772,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":31292,"parent_record_offset":31268,"ancestor_record_offset":30772,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":513,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":30772,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":513,"record_offset":31532,"parent_record_offset":31508,"ancestor_record_offset":30772,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":31340,"parent_record_offset":31316,"ancestor_record_offset":30772,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":31364,"parent_record_offset":31340,"ancestor_record_offset":30772,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":32300,"parent_record_offset":32272,"ancestor_record_offset":30772,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":31508,"parent_record_offset":31108,"ancestor_record_offset":30772,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":32172,"parent_record_offset":30928,"ancestor_record_offset":30772,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":513,"record_offset":21180,"rva":9771904,"va":5378481024}
// 编译实例 RVAs: 0x951b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8880501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e88b7e78ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5378481296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140149f50
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8422677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8eb7a7fff","size":5,"soft_fail":false,"target_va":5370060624,"va":5378483296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca2f0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6847272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8d88497ff","size":5,"soft_fail":false,"target_va":5371634416,"va":5378481683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5d80
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6735067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8253b99ff","size":5,"soft_fail":false,"target_va":5371747712,"va":5378482774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3670225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e82fffc7ff","size":5,"soft_fail":false,"target_va":5374811808,"va":5378482028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3400589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8731cccff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378482088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406978e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2861624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8c855d4ff","size":5,"soft_fail":false,"target_va":5375621344,"va":5378482963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140891eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-787175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e819fdf3ff","size":5,"soft_fail":false,"target_va":5377695408,"va":5378482578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408b1fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-655180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8b400f6ff","size":5,"soft_fail":false,"target_va":5377826736,"va":5378481911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140abfb30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1498522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e89add1600","size":5,"soft_fail":false,"target_va":5379980080,"va":5378481553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2137357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e80d9d2000","size":5,"soft_fail":false,"target_va":5380620352,"va":5378482990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2138127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e80fa02000","size":5,"soft_fail":false,"target_va":5380620960,"va":5378482828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cd40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2140766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e85eaa2000","size":5,"soft_fail":false,"target_va":5380623680,"va":5378482909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cfc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2141568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e880ad2000","size":5,"soft_fail":false,"target_va":5380624320,"va":5378482747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2141540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e864ad2000","size":5,"soft_fail":false,"target_va":5380624400,"va":5378482855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33364484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8041afd01","size":5,"soft_fail":false,"target_va":5411847184,"va":5378482695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2293281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e821fe2200","size":5,"soft_fail":false,"target_va":5380775408,"va":5378482122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33058134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e8566df801","size":5,"soft_fail":false,"target_va":5411541225,"va":5378483086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33053999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378481024,"raw_hex":"e82f5df801","size":5,"soft_fail":false,"target_va":5411537104,"va":5378483100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":22628,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":24444,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":25272,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":21824,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":26120,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":29576,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23456,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":22976,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":513,"record_offset":27252,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":513,"record_offset":21392,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":513,"record_offset":21936,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":25672,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":513,"record_offset":21852,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":513,"record_offset":27140,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":7307,"source_line":990,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":23716,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":23236,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/replay_cmd_buffer.rs
//     evidence={"module":513,"record_offset":21456,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":29439,"source_line":198,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::WallHistoryEdit::gather_wall_ids
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":513,"record_offset":23868,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":29441,"source_line":276,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":513,"record_offset":22140,"parent_record_offset":21180,"ancestor_record_offset":21180,"inlinee":4784,"source_line":144,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":26060,"parent_record_offset":26032,"ancestor_record_offset":21180,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":513,"record_offset":26032,"parent_record_offset":25876,"ancestor_record_offset":21180,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":25508,"parent_record_offset":25272,"ancestor_record_offset":21180,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":21180,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::front_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21480,"parent_record_offset":21456,"ancestor_record_offset":21180,"inlinee":28183,"source_line":1632,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21504,"parent_record_offset":21480,"ancestor_record_offset":21180,"inlinee":26750,"source_line":715,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":25316,"parent_record_offset":25272,"ancestor_record_offset":21180,"inlinee":4906,"source_line":1342,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21692,"parent_record_offset":21504,"ancestor_record_offset":21180,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21536,"parent_record_offset":21504,"ancestor_record_offset":21180,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":513,"record_offset":21564,"parent_record_offset":21536,"ancestor_record_offset":21180,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":21624,"parent_record_offset":21592,"ancestor_record_offset":21180,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":21724,"parent_record_offset":21692,"ancestor_record_offset":21180,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":21648,"parent_record_offset":21624,"ancestor_record_offset":21180,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":30044,"parent_record_offset":30016,"ancestor_record_offset":21180,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":30016,"parent_record_offset":29992,"ancestor_record_offset":21180,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":21180,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":21180,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":513,"record_offset":29992,"parent_record_offset":29968,"ancestor_record_offset":21180,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":27496,"parent_record_offset":27460,"ancestor_record_offset":21180,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":513,"record_offset":27460,"parent_record_offset":27436,"ancestor_record_offset":21180,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23056,"parent_record_offset":23032,"ancestor_record_offset":21180,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23032,"parent_record_offset":23000,"ancestor_record_offset":21180,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23536,"parent_record_offset":23512,"ancestor_record_offset":21180,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23512,"parent_record_offset":23480,"ancestor_record_offset":21180,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":29624,"parent_record_offset":29576,"ancestor_record_offset":21180,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":29840,"parent_record_offset":29576,"ancestor_record_offset":21180,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":21180,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":513,"record_offset":23000,"parent_record_offset":22976,"ancestor_record_offset":21180,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":513,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":21180,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":513,"record_offset":29944,"parent_record_offset":29920,"ancestor_record_offset":21180,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":513,"record_offset":21420,"parent_record_offset":21392,"ancestor_record_offset":21180,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":513,"record_offset":22424,"parent_record_offset":22396,"ancestor_record_offset":21180,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":513,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":21180,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":513,"record_offset":22452,"parent_record_offset":22396,"ancestor_record_offset":21180,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":513,"record_offset":22500,"parent_record_offset":22476,"ancestor_record_offset":21180,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":513,"record_offset":27212,"parent_record_offset":27168,"ancestor_record_offset":21180,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":25968,"parent_record_offset":25940,"ancestor_record_offset":21180,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":30072,"parent_record_offset":30044,"ancestor_record_offset":21180,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":513,"record_offset":25400,"parent_record_offset":25372,"ancestor_record_offset":21180,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":513,"record_offset":22368,"parent_record_offset":22340,"ancestor_record_offset":21180,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":25724,"parent_record_offset":25700,"ancestor_record_offset":21180,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":513,"record_offset":21900,"parent_record_offset":21876,"ancestor_record_offset":21180,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":513,"record_offset":26000,"parent_record_offset":25908,"ancestor_record_offset":21180,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":513,"record_offset":22096,"parent_record_offset":22072,"ancestor_record_offset":21180,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":513,"record_offset":27528,"parent_record_offset":27496,"ancestor_record_offset":21180,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":513,"record_offset":27324,"parent_record_offset":27252,"ancestor_record_offset":21180,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":513,"record_offset":22016,"parent_record_offset":21972,"ancestor_record_offset":21180,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":27348,"parent_record_offset":27324,"ancestor_record_offset":21180,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":513,"record_offset":22524,"parent_record_offset":22500,"ancestor_record_offset":21180,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":513,"record_offset":27168,"parent_record_offset":27140,"ancestor_record_offset":21180,"inlinee":7309,"source_line":1013,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":513,"record_offset":23652,"parent_record_offset":23480,"ancestor_record_offset":21180,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":23804,"parent_record_offset":23780,"ancestor_record_offset":21180,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":513,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":21180,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":22072,"parent_record_offset":22048,"ancestor_record_offset":21180,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":23780,"parent_record_offset":23716,"ancestor_record_offset":21180,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":21972,"parent_record_offset":21936,"ancestor_record_offset":21180,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":513,"record_offset":22048,"parent_record_offset":21972,"ancestor_record_offset":21180,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":27556,"parent_record_offset":27496,"ancestor_record_offset":21180,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":513,"record_offset":27588,"parent_record_offset":27460,"ancestor_record_offset":21180,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":513,"record_offset":22584,"parent_record_offset":22224,"ancestor_record_offset":21180,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":513,"record_offset":22224,"parent_record_offset":22196,"ancestor_record_offset":21180,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25908,"parent_record_offset":25876,"ancestor_record_offset":21180,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25820,"parent_record_offset":25796,"ancestor_record_offset":21180,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25876,"parent_record_offset":25820,"ancestor_record_offset":21180,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25848,"parent_record_offset":25820,"ancestor_record_offset":21180,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25940,"parent_record_offset":25908,"ancestor_record_offset":21180,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":513,"record_offset":25796,"parent_record_offset":25772,"ancestor_record_offset":21180,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":513,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":21180,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":513,"record_offset":22196,"parent_record_offset":22168,"ancestor_record_offset":21180,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":513,"record_offset":23896,"parent_record_offset":23868,"ancestor_record_offset":21180,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":513,"record_offset":22168,"parent_record_offset":22140,"ancestor_record_offset":21180,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":513,"record_offset":22280,"parent_record_offset":22252,"ancestor_record_offset":21180,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":513,"record_offset":22396,"parent_record_offset":22340,"ancestor_record_offset":21180,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":513,"record_offset":22252,"parent_record_offset":22224,"ancestor_record_offset":21180,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":513,"record_offset":22340,"parent_record_offset":22308,"ancestor_record_offset":21180,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":513,"record_offset":22308,"parent_record_offset":22280,"ancestor_record_offset":21180,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00513.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::replay_wall::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E43
// 首条位置: {"module":564,"inlinee":32323}
// 编译实例 RVAs: 0xabfaa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-877699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979936,"raw_hex":"e87d9bf2ff","size":5,"soft_fail":false,"target_va":5379102272,"va":5379979966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5623495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379979936,"raw_hex":"e83931aaff","size":5,"soft_fail":false,"target_va":5374356528,"va":5379980018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":564,"record_offset":115576,"parent_record_offset":115424,"ancestor_record_offset":115424,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":115768,"parent_record_offset":115736,"ancestor_record_offset":115424,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":115816,"parent_record_offset":115792,"ancestor_record_offset":115424,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":115792,"parent_record_offset":115768,"ancestor_record_offset":115424,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":115736,"parent_record_offset":115696,"ancestor_record_offset":115424,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":115696,"parent_record_offset":115660,"ancestor_record_offset":115424,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":115632,"parent_record_offset":115600,"ancestor_record_offset":115424,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":115600,"parent_record_offset":115576,"ancestor_record_offset":115424,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":564,"record_offset":115660,"parent_record_offset":115600,"ancestor_record_offset":115424,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":115856,"parent_record_offset":115696,"ancestor_record_offset":115424,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayCmd::is_single_adjust_height_cmd
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7323
// 首条位置: {"module":572,"inlinee":29475}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19416,"parent_record_offset":19384,"ancestor_record_offset":19384,"inlinee":11302,"source_line":2924,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19528,"parent_record_offset":19500,"ancestor_record_offset":19384,"inlinee":4735,"source_line":769,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":19384,"inlinee":4910,"source_line":685,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19628,"parent_record_offset":19440,"ancestor_record_offset":19384,"inlinee":4911,"source_line":161,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19472,"parent_record_offset":19440,"ancestor_record_offset":19384,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":572,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":19384,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":19560,"parent_record_offset":19528,"ancestor_record_offset":19384,"inlinee":4736,"source_line":294,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":19664,"parent_record_offset":19628,"ancestor_record_offset":19384,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":19584,"parent_record_offset":19560,"ancestor_record_offset":19384,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":19712,"parent_record_offset":19688,"ancestor_record_offset":19384,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":19688,"parent_record_offset":19664,"ancestor_record_offset":19384,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayCmd::new
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7324
// 首条位置: {"module":1602,"inlinee":29476}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1602,"record_offset":107784,"parent_record_offset":107736,"ancestor_record_offset":107736,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1602,"record_offset":107980,"parent_record_offset":107736,"ancestor_record_offset":107736,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1602,"record_offset":107896,"parent_record_offset":107736,"ancestor_record_offset":107736,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1602,"record_offset":107808,"parent_record_offset":107784,"ancestor_record_offset":107736,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$14::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1602,"record_offset":108004,"parent_record_offset":107980,"ancestor_record_offset":107736,"inlinee":26146,"source_line":2940,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::impl$21::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1602,"record_offset":108112,"parent_record_offset":108080,"ancestor_record_offset":107736,"inlinee":26150,"source_line":3051,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::spec_from_iter::impl$0::spec_from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\spec_from_iter.rs
//     evidence={"module":1602,"record_offset":108056,"parent_record_offset":108004,"ancestor_record_offset":107736,"inlinee":26148,"source_line":13,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_raw_parts_with_alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1602,"record_offset":108136,"parent_record_offset":108112,"ancestor_record_offset":107736,"inlinee":26151,"source_line":1176,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1602,"record_offset":108080,"parent_record_offset":108056,"ancestor_record_offset":107736,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1602,"record_offset":108028,"parent_record_offset":108004,"ancestor_record_offset":107736,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":1602,"record_offset":108160,"parent_record_offset":108136,"ancestor_record_offset":107736,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1602,"record_offset":107944,"parent_record_offset":107920,"ancestor_record_offset":107736,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1602,"record_offset":107920,"parent_record_offset":107896,"ancestor_record_offset":107736,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01602.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E3F
// 首条位置: {"module":564,"inlinee":32319}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":564,"record_offset":108192,"parent_record_offset":108164,"ancestor_record_offset":108164,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":108628,"parent_record_offset":108604,"ancestor_record_offset":108164,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":108652,"parent_record_offset":108628,"ancestor_record_offset":108164,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":108604,"parent_record_offset":108428,"ancestor_record_offset":108164,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":108372,"parent_record_offset":108344,"ancestor_record_offset":108164,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":108428,"parent_record_offset":108400,"ancestor_record_offset":108164,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":108400,"parent_record_offset":108372,"ancestor_record_offset":108164,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":108464,"parent_record_offset":108428,"ancestor_record_offset":108164,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":108732,"parent_record_offset":108708,"ancestor_record_offset":108164,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":108316,"parent_record_offset":108276,"ancestor_record_offset":108164,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":108344,"parent_record_offset":108316,"ancestor_record_offset":108164,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":108220,"parent_record_offset":108192,"ancestor_record_offset":108164,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":564,"record_offset":108708,"parent_record_offset":108276,"ancestor_record_offset":108164,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":564,"record_offset":108248,"parent_record_offset":108220,"ancestor_record_offset":108164,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":564,"record_offset":108276,"parent_record_offset":108248,"ancestor_record_offset":108164,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":564,"record_offset":108488,"parent_record_offset":108464,"ancestor_record_offset":108164,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":564,"record_offset":108512,"parent_record_offset":108488,"ancestor_record_offset":108164,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":564,"record_offset":108756,"parent_record_offset":108732,"ancestor_record_offset":108164,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":564,"record_offset":108780,"parent_record_offset":108756,"ancestor_record_offset":108164,"inlinee":6052,"source_line":796,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":564,"record_offset":108536,"parent_record_offset":108512,"ancestor_record_offset":108164,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":564,"record_offset":108560,"parent_record_offset":108536,"ancestor_record_offset":108164,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1207E
// 首条位置: {"module":1610,"inlinee":73854}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1610,"record_offset":54748,"parent_record_offset":54696,"ancestor_record_offset":54696,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1610,"record_offset":54864,"parent_record_offset":54696,"ancestor_record_offset":54696,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1610,"record_offset":54892,"parent_record_offset":54696,"ancestor_record_offset":54696,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1610,"record_offset":54836,"parent_record_offset":54696,"ancestor_record_offset":54696,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1610,"record_offset":54776,"parent_record_offset":54748,"ancestor_record_offset":54696,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1610,"record_offset":54800,"parent_record_offset":54776,"ancestor_record_offset":54696,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":564,"record_offset":115904,"rva":11270960,"va":5379980080}
// 编译实例 RVAs: 0xabfb30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=6
//     target_aliases=12 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5169340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8441fb1ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5379982759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4905614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e87225b5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379987113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e1630
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1960892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e84414e2ff","size":5,"soft_fail":false,"target_va":5378020912,"va":5379981799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e9640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-878914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8be96f2ff","size":5,"soft_fail":false,"target_va":5379102272,"va":5379981181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9610
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e83295feff","size":5,"soft_fail":false,"target_va":5379888656,"va":5379981529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-92750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8b295feff","size":5,"soft_fail":false,"target_va":5379889152,"va":5379981897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa99e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-93458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8ee92feff","size":5,"soft_fail":false,"target_va":5379889632,"va":5379983085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9bc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-94585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8878efeff","size":5,"soft_fail":false,"target_va":5379890112,"va":5379984692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aa9eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8029afeff","size":5,"soft_fail":false,"target_va":5379890864,"va":5379982505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa090
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8719efeff","size":5,"soft_fail":false,"target_va":5379891344,"va":5379981850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa350
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8439dfeff","size":5,"soft_fail":false,"target_va":5379892048,"va":5379982856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8ce9efeff","size":5,"soft_fail":false,"target_va":5379892528,"va":5379982941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaa710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-88065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8ffa7feff","size":5,"soft_fail":false,"target_va":5379893008,"va":5379981068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140aaaac0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-87050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8f6abfeff","size":5,"soft_fail":false,"target_va":5379893952,"va":5379980997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1953385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e869ce1d00","size":5,"soft_fail":false,"target_va":5381934528,"va":5379981138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f78c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8616842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e88a7b8300","size":5,"soft_fail":false,"target_va":5388597440,"va":5379980593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7970
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8617037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e84d7c8300","size":5,"soft_fail":false,"target_va":5388597616,"va":5379980574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10117993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e869639a00","size":5,"soft_fail":false,"target_va":5390101424,"va":5379983426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: replay_wall.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10118047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e89f639a00","size":5,"soft_fail":false,"target_va":5390103152,"va":5379985100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5624866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8de2baaff","size":5,"soft_fail":false,"target_va":5374355952,"va":5379980813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8810634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e88a708600","size":5,"soft_fail":false,"target_va":5388792336,"va":5379981697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31555282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8d27ee101","size":5,"soft_fail":false,"target_va":5411542064,"va":5379986777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31555033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8d97de101","size":5,"soft_fail":false,"target_va":5411541904,"va":5379986866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::panic_deprecated_operation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-255925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e84b18fcff","size":5,"soft_fail":false,"target_va":5379730976,"va":5379986896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2518272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e80093d9ff","size":5,"soft_fail":false,"target_va":5377467552,"va":5379985819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2513713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8cfa4d9ff","size":5,"soft_fail":false,"target_va":5377469024,"va":5379982732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::_deprecated_morph_curve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1496900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8bc28e9ff","size":5,"soft_fail":false,"target_va":5378483568,"va":5379980463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e88cf1ffff","size":5,"soft_fail":false,"target_va":5379979936,"va":5379983631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7595862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e856e77300","size":5,"soft_fail":false,"target_va":5387580080,"va":5379984213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12792524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8cc32c300","size":5,"soft_fail":false,"target_va":5392773936,"va":5379981407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1922391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e857551d00","size":5,"soft_fail":false,"target_va":5381906160,"va":5379983764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::cubic_in_out
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1926616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8d8651d00","size":5,"soft_fail":false,"target_va":5381906912,"va":5379980291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1932004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8e47a1d00","size":5,"soft_fail":false,"target_va":5381913840,"va":5379981831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1933611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e82b811d00","size":5,"soft_fail":false,"target_va":5381914992,"va":5379981376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1936373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e8f58b1d00","size":5,"soft_fail":false,"target_va":5381917936,"va":5379981558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1951506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379980080,"raw_hex":"e812c71d00","size":5,"soft_fail":false,"target_va":5381932848,"va":5379981337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":564,"record_offset":116140,"parent_record_offset":115904,"ancestor_record_offset":115904,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":116172,"parent_record_offset":115904,"ancestor_record_offset":115904,"inlinee":32320,"source_line":618,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":119368,"parent_record_offset":115904,"ancestor_record_offset":115904,"inlinee":32321,"source_line":307,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":116832,"parent_record_offset":116808,"ancestor_record_offset":115904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":116808,"parent_record_offset":116716,"ancestor_record_offset":115904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117176,"parent_record_offset":117144,"ancestor_record_offset":115904,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116744,"parent_record_offset":116716,"ancestor_record_offset":115904,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116716,"parent_record_offset":116692,"ancestor_record_offset":115904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117224,"parent_record_offset":117200,"ancestor_record_offset":115904,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117200,"parent_record_offset":117176,"ancestor_record_offset":115904,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116692,"parent_record_offset":116668,"ancestor_record_offset":115904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":117144,"parent_record_offset":117104,"ancestor_record_offset":115904,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":117104,"parent_record_offset":117068,"ancestor_record_offset":115904,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":117040,"parent_record_offset":117004,"ancestor_record_offset":115904,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":117004,"parent_record_offset":116980,"ancestor_record_offset":115904,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":564,"record_offset":117068,"parent_record_offset":117004,"ancestor_record_offset":115904,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":564,"record_offset":116980,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":564,"record_offset":117880,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":564,"record_offset":119308,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":564,"record_offset":116776,"parent_record_offset":116744,"ancestor_record_offset":115904,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":118108,"parent_record_offset":118000,"ancestor_record_offset":115904,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":118052,"parent_record_offset":118000,"ancestor_record_offset":115904,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":116940,"parent_record_offset":116916,"ancestor_record_offset":115904,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":119024,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":121856,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":119804,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":116644,"parent_record_offset":116600,"ancestor_record_offset":115904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":117264,"parent_record_offset":117104,"ancestor_record_offset":115904,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":564,"record_offset":118996,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":121916,"parent_record_offset":121892,"ancestor_record_offset":115904,"inlinee":27464,"source_line":239,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":116540,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::_deprecated_update_skirt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121120,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26547,"source_line":241,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117464,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":26536,"source_line":148,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":120612,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26537,"source_line":154,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117372,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":26535,"source_line":142,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_freehand_ty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121232,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26541,"source_line":178,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121148,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26540,"source_line":172,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121204,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26538,"source_line":160,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_supports
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":119748,"parent_record_offset":119368,"ancestor_record_offset":115904,"inlinee":26539,"source_line":166,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":119280,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":26534,"source_line":136,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":118336,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117936,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":26544,"source_line":196,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":121892,"parent_record_offset":121856,"ancestor_record_offset":115904,"inlinee":32325,"source_line":353,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":122076,"parent_record_offset":122040,"ancestor_record_offset":115904,"inlinee":32326,"source_line":502,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":117520,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":32323,"source_line":72,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":117964,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":29540,"source_line":104,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":117492,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":27988,"source_line":82,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":116884,"parent_record_offset":116540,"ancestor_record_offset":115904,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":564,"record_offset":116916,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":564,"record_offset":117904,"parent_record_offset":117880,"ancestor_record_offset":115904,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121944,"parent_record_offset":121916,"ancestor_record_offset":115904,"inlinee":27441,"source_line":176,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":564,"record_offset":118140,"parent_record_offset":116172,"ancestor_record_offset":115904,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121968,"parent_record_offset":121944,"ancestor_record_offset":115904,"inlinee":27443,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":116600,"parent_record_offset":116540,"ancestor_record_offset":115904,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 307
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E41
// 首条位置: {"module":564,"inlinee":32321}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":121856,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":119804,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":121064,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":120228,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::_deprecated_update_skirt
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121120,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26547,"source_line":241,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121092,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26536,"source_line":148,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":120612,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26537,"source_line":154,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121176,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26535,"source_line":142,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_freehand_ty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121232,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26541,"source_line":178,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121148,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26540,"source_line":172,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_style
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121204,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26538,"source_line":160,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_supports
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":119748,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26539,"source_line":166,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121796,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26534,"source_line":136,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121740,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":121712,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":26544,"source_line":196,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":121824,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":32323,"source_line":72,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":119776,"parent_record_offset":119368,"ancestor_record_offset":119368,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":120140,"parent_record_offset":120116,"ancestor_record_offset":119368,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":120116,"parent_record_offset":120024,"ancestor_record_offset":119368,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":120052,"parent_record_offset":120024,"ancestor_record_offset":119368,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":120024,"parent_record_offset":120000,"ancestor_record_offset":119368,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":120000,"parent_record_offset":119976,"ancestor_record_offset":119368,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":564,"record_offset":120084,"parent_record_offset":120052,"ancestor_record_offset":119368,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":119952,"parent_record_offset":119908,"ancestor_record_offset":119368,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":121916,"parent_record_offset":121892,"ancestor_record_offset":119368,"inlinee":27464,"source_line":239,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":119844,"parent_record_offset":119804,"ancestor_record_offset":119368,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":121892,"parent_record_offset":121856,"ancestor_record_offset":119368,"inlinee":32325,"source_line":353,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":122076,"parent_record_offset":122040,"ancestor_record_offset":119368,"inlinee":32326,"source_line":502,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":120192,"parent_record_offset":119844,"ancestor_record_offset":119368,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121944,"parent_record_offset":121916,"ancestor_record_offset":119368,"inlinee":27441,"source_line":176,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121968,"parent_record_offset":121944,"ancestor_record_offset":119368,"inlinee":27443,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":119908,"parent_record_offset":119844,"ancestor_record_offset":119368,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 353
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E45
// 首条位置: {"module":564,"inlinee":32325}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":121916,"parent_record_offset":121892,"ancestor_record_offset":121892,"inlinee":27464,"source_line":239,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121944,"parent_record_offset":121916,"ancestor_record_offset":121892,"inlinee":27441,"source_line":176,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":121968,"parent_record_offset":121944,"ancestor_record_offset":121892,"inlinee":27443,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 502
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E46
// 首条位置: {"module":564,"inlinee":32326}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::_deprecated_empty_w_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":122100,"parent_record_offset":122076,"ancestor_record_offset":122076,"inlinee":27464,"source_line":239,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::_deprecated_new_empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":122128,"parent_record_offset":122100,"ancestor_record_offset":122076,"inlinee":27441,"source_line":176,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$3::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":122152,"parent_record_offset":122128,"ancestor_record_offset":122076,"inlinee":27443,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 618
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E40
// 首条位置: {"module":564,"inlinee":32320}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":564,"record_offset":116980,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":564,"record_offset":117880,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":564,"record_offset":119308,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":564,"record_offset":119024,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":564,"record_offset":118996,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$25::clone
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":564,"record_offset":116540,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":27919,"source_line":84,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117464,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":26536,"source_line":148,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_elevation
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117372,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":26535,"source_line":142,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":119280,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":26534,"source_line":136,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":118336,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":26542,"source_line":184,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":564,"record_offset":117936,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":26544,"source_line":196,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::history::replay_wall::replay_wall::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/history/replay_wall.rs
//     evidence={"module":564,"record_offset":117520,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":32323,"source_line":72,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":117964,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":29540,"source_line":104,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":117492,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":27988,"source_line":82,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":564,"record_offset":116916,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":564,"record_offset":118140,"parent_record_offset":116172,"ancestor_record_offset":116172,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":116832,"parent_record_offset":116808,"ancestor_record_offset":116172,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":564,"record_offset":116808,"parent_record_offset":116716,"ancestor_record_offset":116172,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117176,"parent_record_offset":117144,"ancestor_record_offset":116172,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116744,"parent_record_offset":116716,"ancestor_record_offset":116172,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116716,"parent_record_offset":116692,"ancestor_record_offset":116172,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117224,"parent_record_offset":117200,"ancestor_record_offset":116172,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":117200,"parent_record_offset":117176,"ancestor_record_offset":116172,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":564,"record_offset":116692,"parent_record_offset":116668,"ancestor_record_offset":116172,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":117144,"parent_record_offset":117104,"ancestor_record_offset":116172,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":564,"record_offset":117104,"parent_record_offset":117068,"ancestor_record_offset":116172,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":117040,"parent_record_offset":117004,"ancestor_record_offset":116172,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":564,"record_offset":117004,"parent_record_offset":116980,"ancestor_record_offset":116172,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":564,"record_offset":117068,"parent_record_offset":117004,"ancestor_record_offset":116172,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":564,"record_offset":117548,"parent_record_offset":117520,"ancestor_record_offset":116172,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":564,"record_offset":116776,"parent_record_offset":116744,"ancestor_record_offset":116172,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":118108,"parent_record_offset":118000,"ancestor_record_offset":116172,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":118052,"parent_record_offset":118000,"ancestor_record_offset":116172,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":564,"record_offset":116940,"parent_record_offset":116916,"ancestor_record_offset":116172,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":116644,"parent_record_offset":116600,"ancestor_record_offset":116172,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":564,"record_offset":117264,"parent_record_offset":117104,"ancestor_record_offset":116172,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":564,"record_offset":116884,"parent_record_offset":116540,"ancestor_record_offset":116172,"inlinee":27921,"source_line":11,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::circular_f32::impl$6::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/circular_f32.rs
//     evidence={"module":564,"record_offset":117904,"parent_record_offset":117880,"ancestor_record_offset":116172,"inlinee":13934,"source_line":7,"archive":"_pdb/B/modules/00564.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":564,"record_offset":116600,"parent_record_offset":116540,"ancestor_record_offset":116172,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/00564.jsonl.gz"}
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
// 完整结构及原始类型记录: replay_wall.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::history::replay_wall::WallReplayCmd"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::history::replay_wall::WallReplayEditCmd
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>", "constructor": "country_core::systems::history::replay_wall::WallReplayEditCmd::process", "arguments": ["country_core::systems::history::replay_wall::replay_wall::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>", "core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >", "constructor": "alloc::vec::impl$20::extend_trusted::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::systems::history::replay_wall::WallReplayEditCmd", "core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >", "country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: replay_wall.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-history-owners/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/systems/history/replay_wall.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-integrate/B-consumers/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::systems::history::replay_wall::WallReplayEditCmd", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: replay_wall.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::backend::icc::ActiveIccProfile>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::icc::IccProfileChanged>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::history::history_process::NewReplayCommandEvent>
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::playermade_stairs_audio::PlayStairsAudioCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayCmd::is_single_adjust_height_cmd
// 信息 ID: ef8d6cc87197c69afb9aee21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::replay_edit; module=572, inline_record=19384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayCmd::new
// 信息 ID: 66c2e72d5e3f6cec64a0e31c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>; module=1602, inline_record=107736; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0>; module=1602, inline_record=124976; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_path::generate_path_replay_cmds::closure_env$0>; module=1602, inline_record=142228; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_water::replay_water::closure$1::closure_env$0>; module=1602, inline_record=159480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>
// 信息 ID: b062a05856d4599629657259
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::systems::history::replay_wall::WallReplayEditCmd::process -> ["country_core::systems::history::replay_wall::replay_wall::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::replay_wall @ 0x951d91 -> 0xabfb30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated
// 信息 ID: 183eddd66190cb1b18866271
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=116172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant
// 信息 ID: 8b68fa218e72250b9fa6e70d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=119368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::_deprecated_morph_curve
// 信息 ID: 4fdea292f9b2a18def98a7e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0xabfe7b -> 0x952570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0> @ 0xabfcaf -> 0x952570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0
// 信息 ID: 79f33aa8294e0c5feb534482
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global>,core::iter::adapters::map::Map<alloc::boxed::Box<dyn$<core::iter::traits::iterator::Iterator<assoc$<Item,ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> > > >,alloc::alloc::Global>,country_core::systems::history::replay_wall::impl$0::new::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<ref$<country_core::resources::history::wall_compound_edit::ConcurrentEdits> >,country_core::systems::history::replay_wall::impl$0::new::closure_env$0>; module=564, inline_record=108164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0
// 信息 ID: e859d3e806833ac416e733e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$0::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global> >,country_core::systems::history::replay_wall::impl$0::new::closure$0::closure_env$0> > > >; module=1610, inline_record=54696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$0
// 信息 ID: c9c1f19a4fd64b8d864e8862
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=121892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::impl$1::process_instant::closure$1
// 信息 ID: 68646a57122e0ba39417f355
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::history::replay_wall::impl$1::process_instant", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=122076; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall
// 信息 ID: 384c7003d3a4305ea0c5f413
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d8d5b2f07370a771b68d76d5: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0xabe394 -> 0x951b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > > @ 0x14e69a6 -> 0x951b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::history::replay_wall::replay_wall::closure$0
// 信息 ID: 0c69d16c1f5e0e05a5bb4fbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::history::replay_wall::replay_wall", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::history::replay_wall::replay_wall::closure_env$0 <- country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=117520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>; module=564, inline_record=121824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
