// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/egui_backend.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 47333556286f9eef47fa5141b13dddc5555cdb3e616752da54362b49c35f4ea1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: egui_backend.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 33, 194, 240
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
// use ecolor; // namespace candidate
//   package identities: [{"name":"ecolor","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use epaint; // namespace candidate
//   package identities: [{"name":"epaint","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2069
// 首条位置: {"module":240,"inlinee":8297}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] epaint::textures::impl$35::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/textures.rs
//     evidence={"module":240,"record_offset":75308,"parent_record_offset":74860,"ancestor_record_offset":74860,"inlinee":8245,"source_line":251,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":240,"record_offset":74916,"parent_record_offset":74860,"ancestor_record_offset":74860,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":240,"record_offset":75144,"parent_record_offset":75116,"ancestor_record_offset":74860,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":240,"record_offset":75196,"parent_record_offset":75172,"ancestor_record_offset":74860,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":240,"record_offset":75172,"parent_record_offset":75116,"ancestor_record_offset":74860,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":240,"record_offset":75220,"parent_record_offset":75196,"ancestor_record_offset":74860,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":240,"record_offset":75088,"parent_record_offset":75060,"ancestor_record_offset":74860,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":240,"record_offset":75244,"parent_record_offset":75220,"ancestor_record_offset":74860,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":240,"record_offset":74988,"parent_record_offset":74952,"ancestor_record_offset":74860,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":240,"record_offset":75116,"parent_record_offset":75060,"ancestor_record_offset":74860,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":240,"record_offset":74952,"parent_record_offset":74916,"ancestor_record_offset":74860,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":240,"record_offset":75060,"parent_record_offset":75024,"ancestor_record_offset":74860,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":240,"record_offset":75024,"parent_record_offset":74988,"ancestor_record_offset":74860,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00240.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::take_input
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x206A
// 首条位置: {"module":194,"inlinee":8298}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::end_frame
// 记录形式: compiled_procedure; instances=1
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":33,"record_offset":7392,"rva":907536,"va":5369616656}
// 编译实例 RVAs: 0xdd910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140052830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-569802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e8364ef7ff","size":5,"soft_fail":false,"target_va":5369047088,"va":5369616885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e75e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5282685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e87d9b5000","size":5,"soft_fail":false,"target_va":5374899680,"va":5369616990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee330
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5310159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e8cf065100","size":5,"soft_fail":false,"target_va":5374927664,"va":5369617500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5320867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e8a3305100","size":5,"soft_fail":false,"target_va":5374938384,"va":5369617512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f81c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5350768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e870a55100","size":5,"soft_fail":false,"target_va":5374968256,"va":5369617483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5464463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e88f615300","size":5,"soft_fail":false,"target_va":5375081504,"va":5369617036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::end_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12097455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e8af97b800","size":5,"soft_fail":false,"target_va":5381714160,"va":5369616700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] epaint::textures::TexturesDelta::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12199681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616656,"raw_hex":"e80127ba00","size":5,"soft_fail":false,"target_va":5381816800,"va":5369617114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":7524,"parent_record_offset":7392,"ancestor_record_offset":7392,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":7392,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":7924,"parent_record_offset":7836,"ancestor_record_offset":7392,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7392,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":7392,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":7392,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":7668,"parent_record_offset":7640,"ancestor_record_offset":7392,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":33,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":7392,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":7548,"parent_record_offset":7524,"ancestor_record_offset":7392,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":7392,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::draw
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":33,"record_offset":8204,"rva":908416,"va":5369617536}
// 编译实例 RVAs: 0xddc80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-669768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8b8c7f5ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5369623491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140041e40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-642063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8f133f6ff","size":5,"soft_fail":false,"target_va":5368979008,"va":5369621066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140053470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-573109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e84b41f7ff","size":5,"soft_fail":false,"target_va":5369050224,"va":5369623328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-250944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8c02bfcff","size":5,"soft_fail":false,"target_va":5369374080,"va":5369625019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcef0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e873d4ffff","size":5,"soft_fail":false,"target_va":5369614064,"va":5369625208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e849eeffff","size":5,"soft_fail":false,"target_va":5369616112,"va":5369620642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403728a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2701436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e87c382900","size":5,"soft_fail":false,"target_va":5372324000,"va":5369622559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1d60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5061202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8523a4d00","size":5,"soft_fail":false,"target_va":5374680416,"va":5369619209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5092646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e826b54d00","size":5,"soft_fail":false,"target_va":5374712576,"va":5369619925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d21b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5191984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e830394f00","size":5,"soft_fail":false,"target_va":5374812592,"va":5369620603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e94a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5280347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e85b925000","size":5,"soft_fail":false,"target_va":5374907552,"va":5369627200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc780
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5358379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e82bc35100","size":5,"soft_fail":false,"target_va":5374986112,"va":5369627728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140603110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5386185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8c92f5200","size":5,"soft_fail":false,"target_va":5375013136,"va":5369626946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406048f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5391005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e89d425200","size":5,"soft_fail":false,"target_va":5375019248,"va":5369628238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140605ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5401667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8436c5200","size":5,"soft_fail":false,"target_va":5375024800,"va":5369623128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5462709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8b55a5300","size":5,"soft_fail":false,"target_va":5375081488,"va":5369618774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5461260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e80c555300","size":5,"soft_fail":false,"target_va":5375081504,"va":5369620239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5704766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e83e0c5700","size":5,"soft_fail":false,"target_va":5375326672,"va":5369621901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5894976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e840f35900","size":5,"soft_fail":false,"target_va":5375516032,"va":5369621051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11552620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e86c47b000","size":5,"soft_fail":false,"target_va":5381170352,"va":5369617727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beea80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11599014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8a6fcb000","size":5,"soft_fail":false,"target_va":5381220992,"va":5369621973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfe9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11666742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e83605b200","size":5,"soft_fail":false,"target_va":5381286336,"va":5369619589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289e380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41679711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e85ffb7b02","size":5,"soft_fail":false,"target_va":5411300224,"va":5369620508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41694681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8d9357c02","size":5,"soft_fail":false,"target_va":5411313216,"va":5369618530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: egui_backend.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42225099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8cb4d8402","size":5,"soft_fail":false,"target_va":5411847168,"va":5369622064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4733360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8b0394800","size":5,"soft_fail":false,"target_va":5374356144,"va":5369622779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41916859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8bb997f02","size":5,"soft_fail":false,"target_va":5411540963,"va":5369624099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12187279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e88ff6b900","size":5,"soft_fail":false,"target_va":5381807680,"va":5369620396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41917990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8269e7f02","size":5,"soft_fail":false,"target_va":5411541584,"va":5369623589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41918008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8389e7f02","size":5,"soft_fail":false,"target_va":5411541808,"va":5369623795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t41918667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8cba07f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5369623104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41918718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8fea07f02","size":5,"soft_fail":false,"target_va":5411542422,"va":5369623699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41917931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8eb9d7f02","size":5,"soft_fail":false,"target_va":5411542064,"va":5369624128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t41918975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8ffa17f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5369623900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41919530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e82aa47f02","size":5,"soft_fail":false,"target_va":5411543264,"va":5369623729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8f4f0ffff","size":5,"soft_fail":false,"target_va":5369614272,"va":5369618119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41919148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8aca27f02","size":5,"soft_fail":false,"target_va":5411543360,"va":5369624207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8753560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e898918500","size":5,"soft_fail":false,"target_va":5378374304,"va":5369620739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8549281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8a1738200","size":5,"soft_fail":false,"target_va":5378171904,"va":5369622618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao_indexed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8551425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8017c8200","size":5,"soft_fail":false,"target_va":5378174128,"va":5369622698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8547470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e88e6c8200","size":5,"soft_fail":false,"target_va":5378169824,"va":5369622349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7899422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e81e897800","size":5,"soft_fail":false,"target_va":5377521744,"va":5369622317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7901105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8b18f7800","size":5,"soft_fail":false,"target_va":5377522032,"va":5369620922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7901412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8e4907800","size":5,"soft_fail":false,"target_va":5377524592,"va":5369623175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8563106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8a2a98200","size":5,"soft_fail":false,"target_va":5378183920,"va":5369620809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::tessellate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12101383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e807a7b800","size":5,"soft_fail":false,"target_va":5381722560,"va":5369621172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11766924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e88c8cb300","size":5,"soft_fail":false,"target_va":5381385536,"va":5369618607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11772144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8f0a0b300","size":5,"soft_fail":false,"target_va":5381390640,"va":5369618491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11766133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e87589b300","size":5,"soft_fail":false,"target_va":5381386480,"va":5369620342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11766594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8428bb300","size":5,"soft_fail":false,"target_va":5381384704,"va":5369618105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11493543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8a760af00","size":5,"soft_fail":false,"target_va":5381115488,"va":5369621940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11497058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8626eaf00","size":5,"soft_fail":false,"target_va":5381117840,"va":5369620777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11496800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8606daf00","size":5,"soft_fail":false,"target_va":5381116576,"va":5369619771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11496949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e8f56daf00","size":5,"soft_fail":false,"target_va":5381116784,"va":5369619830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41913182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617536,"raw_hex":"e85e8b7f02","size":5,"soft_fail":false,"target_va":5411537104,"va":5369623917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":8416,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":20080,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":23188,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":19832,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":33,"record_offset":19920,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":33,"record_offset":17920,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":33,"record_offset":19716,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":18700,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":21164,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":33,"record_offset":21908,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":33,"record_offset":22008,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::DrawBuilder::push_constants
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":33,"record_offset":22300,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8281,"source_line":127,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":33,"record_offset":22972,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8283,"source_line":524,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":33,"record_offset":18764,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8275,"source_line":337,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] egui::context::Context::pixels_per_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":33,"record_offset":19660,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":6384,"source_line":1595,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":33,"record_offset":21952,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23100,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8289,"source_line":637,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23016,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8285,"source_line":628,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":33,"record_offset":18736,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":20136,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::release_pending_textures
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":33,"record_offset":24524,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8291,"source_line":218,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::update_textures
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":33,"record_offset":8796,"parent_record_offset":8204,"ancestor_record_offset":8204,"inlinee":8247,"source_line":164,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":8204,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":8204,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":8204,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12520,"parent_record_offset":12480,"ancestor_record_offset":8204,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":15892,"parent_record_offset":15808,"ancestor_record_offset":8204,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":8500,"parent_record_offset":8476,"ancestor_record_offset":8204,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":8204,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15836,"parent_record_offset":15808,"ancestor_record_offset":8204,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":8204,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":8204,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":8524,"parent_record_offset":8500,"ancestor_record_offset":8204,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12480,"parent_record_offset":12440,"ancestor_record_offset":8204,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12440,"parent_record_offset":12404,"ancestor_record_offset":8204,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15784,"parent_record_offset":15760,"ancestor_record_offset":8204,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":33,"record_offset":22444,"parent_record_offset":22412,"ancestor_record_offset":8204,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":33,"record_offset":22412,"parent_record_offset":22380,"ancestor_record_offset":8204,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":33,"record_offset":22348,"parent_record_offset":22300,"ancestor_record_offset":8204,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":33,"record_offset":22380,"parent_record_offset":22348,"ancestor_record_offset":8204,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":33,"record_offset":17168,"parent_record_offset":17132,"ancestor_record_offset":8204,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":33,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":8204,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":8476,"parent_record_offset":8452,"ancestor_record_offset":8204,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":8452,"parent_record_offset":8416,"ancestor_record_offset":8204,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9028,"parent_record_offset":8988,"ancestor_record_offset":8204,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12160,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":8988,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":8204,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9140,"parent_record_offset":8988,"ancestor_record_offset":8204,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9168,"parent_record_offset":8988,"ancestor_record_offset":8204,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12332,"parent_record_offset":12284,"ancestor_record_offset":8204,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12368,"parent_record_offset":12332,"ancestor_record_offset":8204,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":9284,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":8204,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12220,"parent_record_offset":12188,"ancestor_record_offset":8204,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":8204,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":18260,"parent_record_offset":18236,"ancestor_record_offset":8204,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":33,"record_offset":13140,"parent_record_offset":13076,"ancestor_record_offset":8204,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":33,"record_offset":12764,"parent_record_offset":12284,"ancestor_record_offset":8204,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":33,"record_offset":12252,"parent_record_offset":12220,"ancestor_record_offset":8204,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":33,"record_offset":12284,"parent_record_offset":12252,"ancestor_record_offset":8204,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":33,"record_offset":18648,"parent_record_offset":18624,"ancestor_record_offset":8204,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":33,"record_offset":18624,"parent_record_offset":18600,"ancestor_record_offset":8204,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":33,"record_offset":18572,"parent_record_offset":18540,"ancestor_record_offset":8204,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":33,"record_offset":18516,"parent_record_offset":17984,"ancestor_record_offset":8204,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":33,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":8204,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":33,"record_offset":18012,"parent_record_offset":17984,"ancestor_record_offset":8204,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":33,"record_offset":18600,"parent_record_offset":18540,"ancestor_record_offset":8204,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":33,"record_offset":18060,"parent_record_offset":18036,"ancestor_record_offset":8204,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":33,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":8204,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":33,"record_offset":18236,"parent_record_offset":18212,"ancestor_record_offset":8204,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":33,"record_offset":17948,"parent_record_offset":17920,"ancestor_record_offset":8204,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":33,"record_offset":17984,"parent_record_offset":17920,"ancestor_record_offset":8204,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":8204,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":8204,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":8204,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11352,"parent_record_offset":11320,"ancestor_record_offset":8204,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11384,"parent_record_offset":11352,"ancestor_record_offset":8204,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11556,"parent_record_offset":11384,"ancestor_record_offset":8204,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":8204,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":33,"record_offset":20920,"parent_record_offset":20896,"ancestor_record_offset":8204,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":33,"record_offset":18796,"parent_record_offset":18764,"ancestor_record_offset":8204,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":8204,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":8204,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":8204,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":8204,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":13956,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":14176,"parent_record_offset":14144,"ancestor_record_offset":8204,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12848,"parent_record_offset":12820,"ancestor_record_offset":8204,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12624,"parent_record_offset":12284,"ancestor_record_offset":8204,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12912,"parent_record_offset":12876,"ancestor_record_offset":8204,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":12188,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":8204,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":13048,"parent_record_offset":12912,"ancestor_record_offset":8204,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":33,"record_offset":19860,"parent_record_offset":19832,"ancestor_record_offset":8204,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":33,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":8204,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":33,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":8204,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":33,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":8204,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":33,"record_offset":15488,"parent_record_offset":15460,"ancestor_record_offset":8204,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":33,"record_offset":9740,"parent_record_offset":9708,"ancestor_record_offset":8204,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":33,"record_offset":13744,"parent_record_offset":13716,"ancestor_record_offset":8204,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":33,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":8204,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":33,"record_offset":11188,"parent_record_offset":11148,"ancestor_record_offset":8204,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":18172,"parent_record_offset":18148,"ancestor_record_offset":8204,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":25280,"parent_record_offset":24524,"ancestor_record_offset":8204,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":9472,"parent_record_offset":9284,"ancestor_record_offset":8204,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":18916,"parent_record_offset":18888,"ancestor_record_offset":8204,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":8204,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":11288,"parent_record_offset":11148,"ancestor_record_offset":8204,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":22696,"parent_record_offset":22444,"ancestor_record_offset":8204,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":12720,"parent_record_offset":12696,"ancestor_record_offset":8204,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":8592,"parent_record_offset":8416,"ancestor_record_offset":8204,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8204,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":8204,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":12696,"parent_record_offset":12672,"ancestor_record_offset":8204,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":8204,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":8204,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":17200,"parent_record_offset":17168,"ancestor_record_offset":8204,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":8204,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":20020,"parent_record_offset":19920,"ancestor_record_offset":8204,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":8204,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":8204,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":9352,"parent_record_offset":9308,"ancestor_record_offset":8204,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":8204,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":8204,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":33,"record_offset":14332,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":33,"record_offset":18088,"parent_record_offset":18060,"ancestor_record_offset":8204,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":33,"record_offset":9196,"parent_record_offset":8988,"ancestor_record_offset":8204,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":33,"record_offset":18464,"parent_record_offset":18440,"ancestor_record_offset":8204,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":33,"record_offset":18120,"parent_record_offset":18088,"ancestor_record_offset":8204,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":33,"record_offset":18440,"parent_record_offset":18260,"ancestor_record_offset":8204,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":33,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":8204,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":8204,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":8204,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":8204,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10716,"parent_record_offset":10688,"ancestor_record_offset":8204,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":12876,"parent_record_offset":12848,"ancestor_record_offset":8204,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":8204,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":8204,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10364,"parent_record_offset":10248,"ancestor_record_offset":8204,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9384,"parent_record_offset":9308,"ancestor_record_offset":8204,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":33,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":8204,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":33,"record_offset":10076,"parent_record_offset":10040,"ancestor_record_offset":8204,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":33,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":8204,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":33,"record_offset":10184,"parent_record_offset":10148,"ancestor_record_offset":8204,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":8204,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":8204,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":8204,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10688,"parent_record_offset":10148,"ancestor_record_offset":8204,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9680,"parent_record_offset":9644,"ancestor_record_offset":8204,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9644,"parent_record_offset":9612,"ancestor_record_offset":8204,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9844,"parent_record_offset":9812,"ancestor_record_offset":8204,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9936,"parent_record_offset":9844,"ancestor_record_offset":8204,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9812,"parent_record_offset":9784,"ancestor_record_offset":8204,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9612,"parent_record_offset":9560,"ancestor_record_offset":8204,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":8204,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10248,"parent_record_offset":10216,"ancestor_record_offset":8204,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10532,"parent_record_offset":10248,"ancestor_record_offset":8204,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10504,"parent_record_offset":10248,"ancestor_record_offset":8204,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10308,"parent_record_offset":10248,"ancestor_record_offset":8204,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":33,"record_offset":17228,"parent_record_offset":17168,"ancestor_record_offset":8204,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":33,"record_offset":17260,"parent_record_offset":17132,"ancestor_record_offset":8204,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":33,"record_offset":18844,"parent_record_offset":18820,"ancestor_record_offset":8204,"inlinee":8279,"source_line":145,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":33,"record_offset":18820,"parent_record_offset":18796,"ancestor_record_offset":8204,"inlinee":8277,"source_line":179,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::Color32::from_rgba_premultiplied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ecolor","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":33,"record_offset":13012,"parent_record_offset":12940,"ancestor_record_offset":8204,"inlinee":8263,"source_line":89,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] egui::context::Context::input
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.27.2/src/context.rs
//     evidence={"module":33,"record_offset":19684,"parent_record_offset":19660,"ancestor_record_offset":8204,"inlinee":6385,"source_line":795,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::height
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12096,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":8255,"source_line":33,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::size
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12060,"parent_record_offset":12032,"ancestor_record_offset":8204,"inlinee":8254,"source_line":22,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12032,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":8253,"source_line":29,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::fast_round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12980,"parent_record_offset":12940,"ancestor_record_offset":8204,"inlinee":8261,"source_line":359,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::impl$7::srgba_pixels::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12940,"parent_record_offset":12912,"ancestor_record_offset":8204,"inlinee":8259,"source_line":305,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::impl$5::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/lib.rs
//     evidence={"module":33,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":8204,"inlinee":6229,"source_line":86,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":8204,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":8204,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13628,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13488,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":15204,"parent_record_offset":15132,"ancestor_record_offset":8204,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13984,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":14976,"parent_record_offset":14936,"ancestor_record_offset":8204,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13572,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":33,"record_offset":13432,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":33,"record_offset":15612,"parent_record_offset":14664,"ancestor_record_offset":8204,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":33,"record_offset":20896,"parent_record_offset":20872,"ancestor_record_offset":8204,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":20160,"parent_record_offset":20136,"ancestor_record_offset":8204,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":20184,"parent_record_offset":20160,"ancestor_record_offset":8204,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":8204,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":25212,"parent_record_offset":25184,"ancestor_record_offset":8204,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":14116,"parent_record_offset":13280,"ancestor_record_offset":8204,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":13280,"parent_record_offset":13244,"ancestor_record_offset":8204,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":20872,"parent_record_offset":20704,"ancestor_record_offset":8204,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":13324,"parent_record_offset":13280,"ancestor_record_offset":8204,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":20812,"parent_record_offset":20728,"ancestor_record_offset":8204,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":8204,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15640,"parent_record_offset":14664,"ancestor_record_offset":8204,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14076,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":8204,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13352,"parent_record_offset":13280,"ancestor_record_offset":8204,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":20244,"parent_record_offset":20184,"ancestor_record_offset":8204,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14664,"parent_record_offset":14632,"ancestor_record_offset":8204,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":8204,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15528,"parent_record_offset":14664,"ancestor_record_offset":8204,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":8204,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14752,"parent_record_offset":14712,"ancestor_record_offset":8204,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14936,"parent_record_offset":14752,"ancestor_record_offset":8204,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15132,"parent_record_offset":14752,"ancestor_record_offset":8204,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15168,"parent_record_offset":15132,"ancestor_record_offset":8204,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14712,"parent_record_offset":14664,"ancestor_record_offset":8204,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13460,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":8204,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15436,"parent_record_offset":14712,"ancestor_record_offset":8204,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13788,"parent_record_offset":13376,"ancestor_record_offset":8204,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":8204,"inlinee":8270,"source_line":350,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":14632,"parent_record_offset":14600,"ancestor_record_offset":8204,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":13896,"parent_record_offset":13788,"ancestor_record_offset":8204,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":33,"record_offset":23064,"parent_record_offset":23040,"ancestor_record_offset":8204,"inlinee":8288,"source_line":64,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11024,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11064,"parent_record_offset":11024,"ancestor_record_offset":8204,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":9560,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":16044,"parent_record_offset":16004,"ancestor_record_offset":8204,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":16612,"parent_record_offset":16340,"ancestor_record_offset":8204,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":33,"record_offset":11216,"parent_record_offset":11188,"ancestor_record_offset":8204,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11320,"parent_record_offset":11148,"ancestor_record_offset":8204,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11592,"parent_record_offset":11320,"ancestor_record_offset":8204,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":10040,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":33,"record_offset":14220,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23124,"parent_record_offset":23100,"ancestor_record_offset":8204,"inlinee":8290,"source_line":637,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":33,"record_offset":23040,"parent_record_offset":23016,"ancestor_record_offset":8204,"inlinee":8286,"source_line":628,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":13244,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":25160,"parent_record_offset":24524,"ancestor_record_offset":8204,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":14276,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":8268,"source_line":3023,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":14600,"parent_record_offset":8796,"ancestor_record_offset":8204,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":33,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":8204,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":33,"record_offset":11148,"parent_record_offset":11104,"ancestor_record_offset":8204,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":33,"record_offset":11104,"parent_record_offset":11064,"ancestor_record_offset":8204,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::update_textures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2037
// 首条位置: {"module":33,"inlinee":8247}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12160,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":8988,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":9284,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":12188,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":15736,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":33,"record_offset":14332,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] epaint::image::ImageData::height
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12096,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":8255,"source_line":33,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] epaint::image::ImageData::width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12032,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":8253,"source_line":29,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11024,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":9560,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":10040,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":33,"record_offset":14220,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":13244,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::OccupiedEntry::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":14276,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":8268,"source_line":3023,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":14600,"parent_record_offset":8796,"ancestor_record_offset":8796,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12544,"parent_record_offset":12520,"ancestor_record_offset":8796,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":8796,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":8796,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":12520,"parent_record_offset":12480,"ancestor_record_offset":8796,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":33,"record_offset":15892,"parent_record_offset":15808,"ancestor_record_offset":8796,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":8796,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12404,"parent_record_offset":12368,"ancestor_record_offset":8796,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15836,"parent_record_offset":15808,"ancestor_record_offset":8796,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15808,"parent_record_offset":15784,"ancestor_record_offset":8796,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":9100,"parent_record_offset":9076,"ancestor_record_offset":8796,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":8796,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12480,"parent_record_offset":12440,"ancestor_record_offset":8796,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":12440,"parent_record_offset":12404,"ancestor_record_offset":8796,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":15784,"parent_record_offset":15760,"ancestor_record_offset":8796,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":33,"record_offset":17168,"parent_record_offset":17132,"ancestor_record_offset":8796,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":33,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":8796,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9028,"parent_record_offset":8988,"ancestor_record_offset":8796,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12792,"parent_record_offset":12764,"ancestor_record_offset":8796,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9140,"parent_record_offset":8988,"ancestor_record_offset":8796,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":9168,"parent_record_offset":8988,"ancestor_record_offset":8796,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12332,"parent_record_offset":12284,"ancestor_record_offset":8796,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12368,"parent_record_offset":12332,"ancestor_record_offset":8796,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":9496,"parent_record_offset":9472,"ancestor_record_offset":8796,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":12220,"parent_record_offset":12188,"ancestor_record_offset":8796,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":13076,"parent_record_offset":13048,"ancestor_record_offset":8796,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":33,"record_offset":13140,"parent_record_offset":13076,"ancestor_record_offset":8796,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":33,"record_offset":12764,"parent_record_offset":12284,"ancestor_record_offset":8796,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":33,"record_offset":12252,"parent_record_offset":12220,"ancestor_record_offset":8796,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":33,"record_offset":12284,"parent_record_offset":12252,"ancestor_record_offset":8796,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":8796,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":8796,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":8796,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11352,"parent_record_offset":11320,"ancestor_record_offset":8796,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11384,"parent_record_offset":11352,"ancestor_record_offset":8796,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11556,"parent_record_offset":11384,"ancestor_record_offset":8796,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":33,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":8796,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":8796,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":8796,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":33,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":8796,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":8796,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":13956,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":33,"record_offset":14176,"parent_record_offset":14144,"ancestor_record_offset":8796,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12848,"parent_record_offset":12820,"ancestor_record_offset":8796,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12624,"parent_record_offset":12284,"ancestor_record_offset":8796,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":33,"record_offset":12912,"parent_record_offset":12876,"ancestor_record_offset":8796,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":8796,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":33,"record_offset":13048,"parent_record_offset":12912,"ancestor_record_offset":8796,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":33,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":8796,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":33,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":8796,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":33,"record_offset":15860,"parent_record_offset":15836,"ancestor_record_offset":8796,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":33,"record_offset":15488,"parent_record_offset":15460,"ancestor_record_offset":8796,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":33,"record_offset":9740,"parent_record_offset":9708,"ancestor_record_offset":8796,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":33,"record_offset":13744,"parent_record_offset":13716,"ancestor_record_offset":8796,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":33,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":8796,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":33,"record_offset":11188,"parent_record_offset":11148,"ancestor_record_offset":8796,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":9472,"parent_record_offset":9284,"ancestor_record_offset":8796,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":8796,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":11288,"parent_record_offset":11148,"ancestor_record_offset":8796,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":33,"record_offset":12720,"parent_record_offset":12696,"ancestor_record_offset":8796,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":11620,"parent_record_offset":11320,"ancestor_record_offset":8796,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8796,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":13860,"parent_record_offset":13836,"ancestor_record_offset":8796,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":12696,"parent_record_offset":12672,"ancestor_record_offset":8796,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":8796,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":8796,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":17200,"parent_record_offset":17168,"ancestor_record_offset":8796,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":10460,"parent_record_offset":10436,"ancestor_record_offset":8796,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":10436,"parent_record_offset":10412,"ancestor_record_offset":8796,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":12672,"parent_record_offset":12648,"ancestor_record_offset":8796,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":9352,"parent_record_offset":9308,"ancestor_record_offset":8796,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":8796,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":8796,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":33,"record_offset":9196,"parent_record_offset":8988,"ancestor_record_offset":8796,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":33,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":8796,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":8796,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":8796,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":8796,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10716,"parent_record_offset":10688,"ancestor_record_offset":8796,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":12876,"parent_record_offset":12848,"ancestor_record_offset":8796,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":8796,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":8796,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":10364,"parent_record_offset":10248,"ancestor_record_offset":8796,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":9384,"parent_record_offset":9308,"ancestor_record_offset":8796,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":33,"record_offset":9708,"parent_record_offset":9680,"ancestor_record_offset":8796,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":33,"record_offset":10076,"parent_record_offset":10040,"ancestor_record_offset":8796,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":33,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":8796,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":33,"record_offset":10184,"parent_record_offset":10148,"ancestor_record_offset":8796,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":8796,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":8796,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":8796,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":33,"record_offset":10688,"parent_record_offset":10148,"ancestor_record_offset":8796,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9680,"parent_record_offset":9644,"ancestor_record_offset":8796,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9644,"parent_record_offset":9612,"ancestor_record_offset":8796,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9844,"parent_record_offset":9812,"ancestor_record_offset":8796,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9936,"parent_record_offset":9844,"ancestor_record_offset":8796,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9812,"parent_record_offset":9784,"ancestor_record_offset":8796,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9612,"parent_record_offset":9560,"ancestor_record_offset":8796,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":33,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":8796,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10248,"parent_record_offset":10216,"ancestor_record_offset":8796,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10532,"parent_record_offset":10248,"ancestor_record_offset":8796,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10504,"parent_record_offset":10248,"ancestor_record_offset":8796,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":33,"record_offset":10308,"parent_record_offset":10248,"ancestor_record_offset":8796,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":33,"record_offset":17228,"parent_record_offset":17168,"ancestor_record_offset":8796,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":33,"record_offset":17260,"parent_record_offset":17132,"ancestor_record_offset":8796,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] ecolor::color32::Color32::from_rgba_premultiplied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ecolor","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ecolor-0.27.2/src/color32.rs
//     evidence={"module":33,"record_offset":13012,"parent_record_offset":12940,"ancestor_record_offset":8796,"inlinee":8263,"source_line":89,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::ImageData::size
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12060,"parent_record_offset":12032,"ancestor_record_offset":8796,"inlinee":8254,"source_line":22,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::fast_round
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12980,"parent_record_offset":12940,"ancestor_record_offset":8796,"inlinee":8261,"source_line":359,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::image::impl$7::srgba_pixels::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/image.rs
//     evidence={"module":33,"record_offset":12940,"parent_record_offset":12912,"ancestor_record_offset":8796,"inlinee":8259,"source_line":305,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] epaint::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"epaint","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/epaint-0.27.2/src/lib.rs
//     evidence={"module":33,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":8796,"inlinee":6229,"source_line":86,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13656,"parent_record_offset":13628,"ancestor_record_offset":8796,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13688,"parent_record_offset":13656,"ancestor_record_offset":8796,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":33,"record_offset":13628,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13488,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":15204,"parent_record_offset":15132,"ancestor_record_offset":8796,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13984,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":14976,"parent_record_offset":14936,"ancestor_record_offset":8796,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":33,"record_offset":13572,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":33,"record_offset":13432,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":33,"record_offset":15612,"parent_record_offset":14664,"ancestor_record_offset":8796,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":14116,"parent_record_offset":13280,"ancestor_record_offset":8796,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":13280,"parent_record_offset":13244,"ancestor_record_offset":8796,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":13324,"parent_record_offset":13280,"ancestor_record_offset":8796,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13836,"parent_record_offset":13812,"ancestor_record_offset":8796,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15640,"parent_record_offset":14664,"ancestor_record_offset":8796,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14076,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":8796,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13352,"parent_record_offset":13280,"ancestor_record_offset":8796,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14664,"parent_record_offset":14632,"ancestor_record_offset":8796,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14144,"parent_record_offset":14116,"ancestor_record_offset":8796,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15528,"parent_record_offset":14664,"ancestor_record_offset":8796,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":8796,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14752,"parent_record_offset":14712,"ancestor_record_offset":8796,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14936,"parent_record_offset":14752,"ancestor_record_offset":8796,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15132,"parent_record_offset":14752,"ancestor_record_offset":8796,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15168,"parent_record_offset":15132,"ancestor_record_offset":8796,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":14712,"parent_record_offset":14664,"ancestor_record_offset":8796,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13460,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15460,"parent_record_offset":15436,"ancestor_record_offset":8796,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":15436,"parent_record_offset":14712,"ancestor_record_offset":8796,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":33,"record_offset":13788,"parent_record_offset":13376,"ancestor_record_offset":8796,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":8796,"inlinee":8270,"source_line":350,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":14632,"parent_record_offset":14600,"ancestor_record_offset":8796,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":33,"record_offset":13896,"parent_record_offset":13788,"ancestor_record_offset":8796,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11064,"parent_record_offset":11024,"ancestor_record_offset":8796,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":16044,"parent_record_offset":16004,"ancestor_record_offset":8796,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":16612,"parent_record_offset":16340,"ancestor_record_offset":8796,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":33,"record_offset":11216,"parent_record_offset":11188,"ancestor_record_offset":8796,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11320,"parent_record_offset":11148,"ancestor_record_offset":8796,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":33,"record_offset":11592,"parent_record_offset":11320,"ancestor_record_offset":8796,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":33,"record_offset":11244,"parent_record_offset":11216,"ancestor_record_offset":8796,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":33,"record_offset":11148,"parent_record_offset":11104,"ancestor_record_offset":8796,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":33,"record_offset":11104,"parent_record_offset":11064,"ancestor_record_offset":8796,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::egui_backend::EguiBackend::release_pending_textures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2063
// 首条位置: {"module":33,"inlinee":8291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":24580,"parent_record_offset":24524,"ancestor_record_offset":24524,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":24876,"parent_record_offset":24524,"ancestor_record_offset":24524,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":25280,"parent_record_offset":24524,"ancestor_record_offset":24524,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":25320,"parent_record_offset":24524,"ancestor_record_offset":24524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":33,"record_offset":25160,"parent_record_offset":24524,"ancestor_record_offset":24524,"inlinee":8292,"source_line":1244,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":24644,"parent_record_offset":24620,"ancestor_record_offset":24524,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":24692,"parent_record_offset":24668,"ancestor_record_offset":24524,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":33,"record_offset":24668,"parent_record_offset":24644,"ancestor_record_offset":24524,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":24620,"parent_record_offset":24580,"ancestor_record_offset":24524,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":24732,"parent_record_offset":24580,"ancestor_record_offset":24524,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":33,"record_offset":24760,"parent_record_offset":24580,"ancestor_record_offset":24524,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":33,"record_offset":25096,"parent_record_offset":25072,"ancestor_record_offset":24524,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":33,"record_offset":25072,"parent_record_offset":24876,"ancestor_record_offset":24524,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":25348,"parent_record_offset":25320,"ancestor_record_offset":24524,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":33,"record_offset":24836,"parent_record_offset":24812,"ancestor_record_offset":24524,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":25032,"parent_record_offset":25008,"ancestor_record_offset":24524,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":33,"record_offset":24952,"parent_record_offset":24904,"ancestor_record_offset":24524,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":33,"record_offset":25120,"parent_record_offset":25096,"ancestor_record_offset":24524,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":33,"record_offset":24788,"parent_record_offset":24580,"ancestor_record_offset":24524,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":33,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":24524,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":24524,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":24904,"parent_record_offset":24876,"ancestor_record_offset":24524,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":33,"record_offset":24984,"parent_record_offset":24904,"ancestor_record_offset":24524,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":25184,"parent_record_offset":25160,"ancestor_record_offset":24524,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":25212,"parent_record_offset":25184,"ancestor_record_offset":24524,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/00033.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":33,"record_offset":25240,"parent_record_offset":25212,"ancestor_record_offset":24524,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00033.jsonl.gz"}
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
// 完整结构及原始类型记录: egui_backend.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::egui_backend::EguiBackend
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::egui_backend::EguiBackend>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::egui_backend::EguiBackend"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: egui_backend.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "tiny_glade::egui_backend::EguiBackend", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: egui_backend.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::draw
// 信息 ID: a191cd6258ef263adc373439
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x16350a -> 0xddc80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::end_frame
// 信息 ID: a27aab0eab88e64dab96d9c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4de3d2 -> 0xdd910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::new
// 信息 ID: 3560cc47f01688924c5ce27f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=240, inline_record=74860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::release_pending_textures
// 信息 ID: 480472d2e1264a3db0004175
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=24524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::take_input
// 信息 ID: 6e9a7ba9eccb2fae3d08c1f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=112772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::egui_backend::EguiBackend::update_textures
// 信息 ID: fbdde5945e350bdeee638142
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=33, inline_record=8796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
