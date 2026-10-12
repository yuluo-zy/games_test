// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/game_key_bindings.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7658e78de2a3972e365788f836b007aeec26c0d72141f0261dbb6ad5258abd3c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_key_bindings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 146, 155, 474, 486, 505, 1587
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::action_desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":27972,"rva":9030496,"va":5377739616}
// 编译实例 RVAs: 0x89cb60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2658198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739616,"raw_hex":"e86a70d7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377739681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34106218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739616,"raw_hex":"e86a6b0802","size":5,"soft_fail":false,"target_va":5411847168,"va":5377740945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5099445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739616,"raw_hex":"e8b5cf4d00","size":5,"soft_fail":false,"target_va":5382840208,"va":5377740758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t33799744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739616,"raw_hex":"e840be0302","size":5,"soft_fail":false,"target_va":5411540963,"va":5377741214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3147864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739616,"raw_hex":"e858083000","size":5,"soft_fail":false,"target_va":5380887680,"va":5377739811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":31916,"parent_record_offset":27972,"ancestor_record_offset":27972,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::icon
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":486,"record_offset":31256,"parent_record_offset":27972,"ancestor_record_offset":27972,"inlinee":26952,"source_line":368,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::text
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":486,"record_offset":28120,"parent_record_offset":27972,"ancestor_record_offset":27972,"inlinee":25922,"source_line":361,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":28468,"parent_record_offset":28444,"ancestor_record_offset":27972,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":28492,"parent_record_offset":28468,"ancestor_record_offset":27972,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":28444,"parent_record_offset":28416,"ancestor_record_offset":27972,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":30352,"parent_record_offset":30324,"ancestor_record_offset":27972,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":28368,"parent_record_offset":28344,"ancestor_record_offset":27972,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":28416,"parent_record_offset":28392,"ancestor_record_offset":27972,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":27972,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":29524,"parent_record_offset":29500,"ancestor_record_offset":27972,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":28316,"parent_record_offset":28292,"ancestor_record_offset":27972,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":28292,"parent_record_offset":28268,"ancestor_record_offset":27972,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":28244,"parent_record_offset":28220,"ancestor_record_offset":27972,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":28268,"parent_record_offset":28244,"ancestor_record_offset":27972,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":28220,"parent_record_offset":28196,"ancestor_record_offset":27972,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":486,"record_offset":28196,"parent_record_offset":28172,"ancestor_record_offset":27972,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":28172,"parent_record_offset":28148,"ancestor_record_offset":27972,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":486,"record_offset":28344,"parent_record_offset":28316,"ancestor_record_offset":27972,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":29548,"parent_record_offset":29524,"ancestor_record_offset":27972,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":29572,"parent_record_offset":29548,"ancestor_record_offset":27972,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":486,"record_offset":28148,"parent_record_offset":28120,"ancestor_record_offset":27972,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":27972,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":27972,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":486,"record_offset":29596,"parent_record_offset":29572,"ancestor_record_offset":27972,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":486,"record_offset":30324,"parent_record_offset":30228,"ancestor_record_offset":27972,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":28544,"parent_record_offset":28316,"ancestor_record_offset":27972,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::MouseBinding::action_desc
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":24136,"rva":9029792,"va":5377738912}
// 编译实例 RVAs: 0x89c8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2657534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377738912,"raw_hex":"e80273d7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377739017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34108116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377738912,"raw_hex":"e8d4720802","size":5,"soft_fail":false,"target_va":5411847168,"va":5377739047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33801856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377738912,"raw_hex":"e880c60302","size":5,"soft_fail":false,"target_va":5411540963,"va":5377739102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3148697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377738912,"raw_hex":"e8990b3000","size":5,"soft_fail":false,"target_va":5380887680,"va":5377738978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::game_key_bindings::KeyBindingDesc::text
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":486,"record_offset":24280,"parent_record_offset":24136,"ancestor_record_offset":24136,"inlinee":25922,"source_line":361,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24824,"parent_record_offset":24800,"ancestor_record_offset":24136,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":24136,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24800,"parent_record_offset":24624,"ancestor_record_offset":24136,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24568,"parent_record_offset":24540,"ancestor_record_offset":24136,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24624,"parent_record_offset":24596,"ancestor_record_offset":24136,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24596,"parent_record_offset":24568,"ancestor_record_offset":24136,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24660,"parent_record_offset":24624,"ancestor_record_offset":24136,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24508,"parent_record_offset":24480,"ancestor_record_offset":24136,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24480,"parent_record_offset":24452,"ancestor_record_offset":24136,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24424,"parent_record_offset":24396,"ancestor_record_offset":24136,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":24136,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24396,"parent_record_offset":24368,"ancestor_record_offset":24136,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":486,"record_offset":24368,"parent_record_offset":24340,"ancestor_record_offset":24136,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":24340,"parent_record_offset":24312,"ancestor_record_offset":24136,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":486,"record_offset":24540,"parent_record_offset":24508,"ancestor_record_offset":24136,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":24136,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":24136,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":486,"record_offset":24312,"parent_record_offset":24280,"ancestor_record_offset":24136,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":24924,"parent_record_offset":24900,"ancestor_record_offset":24136,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":24136,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":486,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24136,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":24900,"parent_record_offset":24508,"ancestor_record_offset":24136,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::mouse_button_to_string
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":24996,"rva":9030000,"va":5377739120}
// 编译实例 RVAs: 0x89c970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2657681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739120,"raw_hex":"e86f72d7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377739164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5100682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739120,"raw_hex":"e88ad44d00","size":5,"soft_fail":false,"target_va":5382840208,"va":5377739521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33801406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377739120,"raw_hex":"e8bec40302","size":5,"soft_fail":false,"target_va":5411540963,"va":5377739552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":27856,"parent_record_offset":24996,"ancestor_record_offset":24996,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":25140,"parent_record_offset":24996,"ancestor_record_offset":24996,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":25436,"parent_record_offset":25412,"ancestor_record_offset":24996,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":25460,"parent_record_offset":25436,"ancestor_record_offset":24996,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":25412,"parent_record_offset":25384,"ancestor_record_offset":24996,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":27904,"parent_record_offset":27880,"ancestor_record_offset":24996,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":25336,"parent_record_offset":25312,"ancestor_record_offset":24996,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":24996,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":25360,"parent_record_offset":25336,"ancestor_record_offset":24996,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":25284,"parent_record_offset":25260,"ancestor_record_offset":24996,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":25260,"parent_record_offset":25236,"ancestor_record_offset":24996,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":25212,"parent_record_offset":25188,"ancestor_record_offset":24996,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":25236,"parent_record_offset":25212,"ancestor_record_offset":24996,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":25188,"parent_record_offset":25164,"ancestor_record_offset":24996,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":486,"record_offset":25164,"parent_record_offset":25140,"ancestor_record_offset":24996,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":27612,"parent_record_offset":27584,"ancestor_record_offset":24996,"inlinee":14877,"source_line":1043,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":486,"record_offset":25312,"parent_record_offset":25284,"ancestor_record_offset":24996,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":24996,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":486,"record_offset":27880,"parent_record_offset":27856,"ancestor_record_offset":24996,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":25512,"parent_record_offset":25284,"ancestor_record_offset":24996,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3338
// 首条位置: {"module":155,"inlinee":13112}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":155,"record_offset":108140,"parent_record_offset":108116,"ancestor_record_offset":108116,"inlinee":5563,"source_line":221,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":146,"record_offset":9692,"rva":3566144,"va":5372275264}
// 编译实例 RVAs: 0x366a40, 0x8320e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34544308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377302752,"raw_hex":"e8b41a0f02","size":5,"soft_fail":false,"target_va":5411847152,"va":5377302839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":146,"record_offset":9880,"parent_record_offset":9692,"ancestor_record_offset":9692,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00146.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":146,"record_offset":9908,"parent_record_offset":9880,"ancestor_record_offset":9692,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00146.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":146,"record_offset":9936,"parent_record_offset":9908,"ancestor_record_offset":9692,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00146.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":146,"record_offset":9964,"parent_record_offset":9936,"ancestor_record_offset":9692,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00146.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x344A
// 首条位置: {"module":155,"inlinee":13386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":94372,"parent_record_offset":94344,"ancestor_record_offset":94344,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":94408,"parent_record_offset":94372,"ancestor_record_offset":94344,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":94476,"parent_record_offset":94372,"ancestor_record_offset":94344,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":94448,"parent_record_offset":94372,"ancestor_record_offset":94344,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3448
// 首条位置: {"module":155,"inlinee":13384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=80
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":155,"record_offset":94692,"parent_record_offset":93668,"ancestor_record_offset":93668,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=86
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":94620,"parent_record_offset":93668,"ancestor_record_offset":93668,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":93712,"parent_record_offset":93668,"ancestor_record_offset":93668,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":94592,"parent_record_offset":93668,"ancestor_record_offset":93668,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":93956,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":155,"record_offset":94088,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":94516,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":93916,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":155,"record_offset":94476,"parent_record_offset":94372,"ancestor_record_offset":93668,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":155,"record_offset":94056,"parent_record_offset":94032,"ancestor_record_offset":93668,"inlinee":13386,"source_line":4,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":155,"record_offset":93988,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":94372,"parent_record_offset":94344,"ancestor_record_offset":93668,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":155,"record_offset":93752,"parent_record_offset":93712,"ancestor_record_offset":93668,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":94448,"parent_record_offset":94372,"ancestor_record_offset":93668,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":155,"record_offset":94032,"parent_record_offset":93752,"ancestor_record_offset":93668,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/00155.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 4
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":474,"record_offset":73136,"rva":8591168,"va":5377300288}
// 编译实例 RVAs: 0x831740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1413115f0
//     owner=unresolved_namespace; count=37
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11402796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377300288,"raw_hex":"e82cfead00","size":5,"soft_fail":false,"target_va":5388703216,"va":5377300415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141312760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11405346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377300288,"raw_hex":"e82208ae00","size":5,"soft_fail":false,"target_va":5388707680,"va":5377302329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141312a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11405880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377300288,"raw_hex":"e8380aae00","size":5,"soft_fail":false,"target_va":5388708416,"va":5377302531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413132c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_key_bindings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11410235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377300288,"raw_hex":"e83b1bae00","size":5,"soft_fail":false,"target_va":5388710592,"va":5377300352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":474,"record_offset":74568,"parent_record_offset":73136,"ancestor_record_offset":73136,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":474,"record_offset":73352,"parent_record_offset":73136,"ancestor_record_offset":73136,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00474.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":474,"record_offset":74508,"parent_record_offset":73136,"ancestor_record_offset":73136,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":474,"record_offset":74620,"parent_record_offset":74592,"ancestor_record_offset":73136,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/00474.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":474,"record_offset":74592,"parent_record_offset":74568,"ancestor_record_offset":73136,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/00474.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$0::default
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x695B
// 首条位置: {"module":486,"record_offset":11964,"rva":9023760,"va":5377732880}
// 编译实例 RVAs: 0x89b110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindings::assign_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":12088,"rva":9024128,"va":5377733248}
// 编译实例 RVAs: 0x89b280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindings::assign_key
// 记录形式: compiled_procedure; instances=1
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":12232,"rva":9024160,"va":5377733280}
// 编译实例 RVAs: 0x89b2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::game_key_bindings::impl$1::assign_key::closure$0
//     owner=private_source_ownership_candidate; count=35
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_key_bindings.rs
//     evidence={"module":486,"record_offset":12372,"parent_record_offset":12232,"ancestor_record_offset":12232,"inlinee":26951,"source_line":118,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$19::eq
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":486,"record_offset":13052,"parent_record_offset":13024,"ancestor_record_offset":12232,"inlinee":10786,"source_line":192,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$27::eq
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":486,"record_offset":12992,"parent_record_offset":12680,"ancestor_record_offset":12232,"inlinee":10720,"source_line":234,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$23::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13140,"parent_record_offset":13084,"ancestor_record_offset":12232,"inlinee":10788,"source_line":1740,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13112,"parent_record_offset":13084,"ancestor_record_offset":12232,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13024,"parent_record_offset":12992,"ancestor_record_offset":12232,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":486,"record_offset":12680,"parent_record_offset":12372,"ancestor_record_offset":12232,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::impl$1::assign_key::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6947
// 首条位置: {"module":486,"inlinee":26951}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":486,"record_offset":12680,"parent_record_offset":12372,"ancestor_record_offset":12372,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$19::eq
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":486,"record_offset":13052,"parent_record_offset":13024,"ancestor_record_offset":12372,"inlinee":10786,"source_line":192,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$27::eq
//     owner=registry_dependency_definition; count=35
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":486,"record_offset":12992,"parent_record_offset":12680,"ancestor_record_offset":12372,"inlinee":10720,"source_line":234,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$23::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13140,"parent_record_offset":13084,"ancestor_record_offset":12372,"inlinee":10788,"source_line":1740,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13112,"parent_record_offset":13084,"ancestor_record_offset":12372,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":486,"record_offset":13024,"parent_record_offset":12992,"ancestor_record_offset":12372,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::MouseBinding::get_mouse_button
// 记录形式: compiled_procedure; instances=1
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":23996,"rva":9029760,"va":5377738880}
// 编译实例 RVAs: 0x89c880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::text
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 361
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6542
// 首条位置: {"module":486,"inlinee":25922}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":486,"record_offset":24312,"parent_record_offset":24280,"ancestor_record_offset":24280,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":486,"record_offset":30324,"parent_record_offset":30228,"ancestor_record_offset":30228,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24824,"parent_record_offset":24800,"ancestor_record_offset":24280,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":24280,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":486,"record_offset":24800,"parent_record_offset":24624,"ancestor_record_offset":24280,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":486,"record_offset":30352,"parent_record_offset":30324,"ancestor_record_offset":30228,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24568,"parent_record_offset":24540,"ancestor_record_offset":24280,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24624,"parent_record_offset":24596,"ancestor_record_offset":24280,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24596,"parent_record_offset":24568,"ancestor_record_offset":24280,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":486,"record_offset":24660,"parent_record_offset":24624,"ancestor_record_offset":24280,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24508,"parent_record_offset":24480,"ancestor_record_offset":24280,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24480,"parent_record_offset":24452,"ancestor_record_offset":24280,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24424,"parent_record_offset":24396,"ancestor_record_offset":24280,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24452,"parent_record_offset":24424,"ancestor_record_offset":24280,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":486,"record_offset":24396,"parent_record_offset":24368,"ancestor_record_offset":24280,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":486,"record_offset":24368,"parent_record_offset":24340,"ancestor_record_offset":24280,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":486,"record_offset":24340,"parent_record_offset":24312,"ancestor_record_offset":24280,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":486,"record_offset":24540,"parent_record_offset":24508,"ancestor_record_offset":24280,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":24280,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":486,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":24280,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":24924,"parent_record_offset":24900,"ancestor_record_offset":24280,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":486,"record_offset":24756,"parent_record_offset":24732,"ancestor_record_offset":24280,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":486,"record_offset":24732,"parent_record_offset":24708,"ancestor_record_offset":24280,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":486,"record_offset":24900,"parent_record_offset":24508,"ancestor_record_offset":24280,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00486.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBindingDesc::icon
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6948
// 首条位置: {"module":486,"inlinee":26952}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::KeyBinding::get_keycode
// 记录形式: compiled_procedure; instances=1
// 源码行: 423
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":37108,"rva":9032448,"va":5377741568}
// 编译实例 RVAs: 0x89d300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::game_key_bindings::keycode_to_str
// 记录形式: compiled_procedure; instances=1
// 源码行: 468
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":486,"record_offset":37268,"rva":9032848,"va":5377741968}
// 编译实例 RVAs: 0x89d490
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
// 完整结构及原始类型记录: game_key_bindings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::game_key_bindings::KeyBinding", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindingDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::KeyBindings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings>", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "country_core::game_key_bindings::KeyBindings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::game_key_bindings::MouseBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::game_key_bindings::MouseBinding>", "constructor": "core::option::Option", "arguments": ["country_core::game_key_bindings::MouseBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::game_key_bindings::MouseBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > >", "constructor": "core::option::Option", "arguments": ["tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "constructor": "tuple$", "arguments": ["ref$<str$>", "alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>", "enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<str$>,alloc::vec::Vec<country_core::game_key_bindings::KeyBinding,alloc::alloc::Global>,enum2$<core::option::Option<tuple$<ref$<str$>,enum2$<core::option::Option<country_core::game_key_bindings::MouseBinding> > > > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_key_bindings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_key_bindings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::action_desc
// 信息 ID: c117d805306c47fb3c5648de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu @ 0x86b28 -> 0x89cb60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$1 @ 0x507f35 -> 0x89cb60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBinding::get_keycode
// 信息 ID: 69e3800d34573c9234d9f221
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1 @ 0x5041df -> 0x89d300 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$1 @ 0x507fc6 -> 0x89d300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::icon
// 信息 ID: f8174377de46c2cc3cbbc9e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=31256; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=31284; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=31888; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=32000; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=33036; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=34144; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=34172; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=34284; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindingDesc::text
// 信息 ID: b4bcfa03d894b98a60457746
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::MouseBinding::action_desc; module=486, inline_record=24280; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=28120; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=28636; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=29212; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=29716; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=30228; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=31312; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBinding::action_desc; module=486, inline_record=32028; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindings::assign_key
// 信息 ID: 1257653dba53c0701fe31d5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$2 @ 0x50858a -> 0x89b2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::KeyBindings::assign_mouse_button
// 信息 ID: 9a3dc029eaaec3bc636d768e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$6 @ 0x508c47 -> 0x89b280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::MouseBinding::action_desc
// 信息 ID: 316571f3a3eae398e50d4a97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x508754 -> 0x89c8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::MouseBinding::get_mouse_button
// 信息 ID: 0e5a69ca744b68ec7b00cbc5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1 @ 0x5043bc -> 0x89c880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x5087de -> 0x89c880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize
// 信息 ID: a0bff7eea5f2cee05632c393
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor>; module=155, inline_record=108116; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=47384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: 0d4e088afc24d2d397b3bef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x38ffcc -> 0x366a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x4a452a -> 0x366a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x4a4560 -> 0x366a40 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x131ea0e -> 0x8320e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x17dec6a -> 0x8320e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::id::impl$1::deserialize_identifier<country_core::game_key_bindings::_::impl$0::deserialize::__FieldVisitor> @ 0x17deca0 -> 0x8320e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 36f87189331e73ae81056b75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=94056; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=94224; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=94344; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=52452; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=52616; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=52736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: f216c1cc6b109bea2e4a3cdb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=155, inline_record=93668; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=52060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: cdde40b63c28c744c37362a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] country_core::game_key_bindings::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,country_core::game_key_bindings::KeyBindings> @ 0x1312682 -> 0x831740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$0::default
// 信息 ID: 5860d261804049106d06a6d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_key_bindings::_::impl$0::deserialize::__Visitor> @ 0x391177 -> 0x89b110 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor> @ 0x396562 -> 0x89b110 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$4 @ 0x50869e -> 0x89b110 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::game_profile::load_game_profile; module=505, inline_record=19740; 不是独立调用边。
// [PDB 内联宿主] country_core::game_profile::load_game_profile; module=505, inline_record=20956; 不是独立调用边。
// [PDB 内联宿主] country_core::game_profile::load_game_profile; module=505, inline_record=21432; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<country_core::game_profile::_::impl$0::deserialize::__Visitor>; module=1587, inline_record=48092; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::impl$1::assign_key::closure$0
// 信息 ID: 5fd93489ee1556cb42e5525e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::game_key_bindings::impl$1::assign_key", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 35 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=12372; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=13192; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=13480; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=13768; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=14056; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=14344; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=14632; 不是独立调用边。
// [PDB 内联宿主] country_core::game_key_bindings::KeyBindings::assign_key; module=486, inline_record=14920; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::keycode_to_str
// 信息 ID: 6b5b8fd36bf34237428e99ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_controls::key_bindings_sub_menu::closure$1 @ 0x50420a -> 0x89d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$1 @ 0x5080cf -> 0x89d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$1 @ 0x507ff4 -> 0x89d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui::closure$7 @ 0xfc7f27 -> 0x89d490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::game_key_bindings::mouse_button_to_string
// 信息 ID: 0fbc765d6d8dba3e902b08e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x5087ef -> 0x89c970 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_keybindings::closure$5 @ 0x5088ad -> 0x89c970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::render_hints::closure$1 @ 0x228c899 -> 0x89c970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
